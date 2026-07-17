import { onMount } from "solid-js";

import { hideVolume, volume } from "@/stores/volume";

import { AudioService } from "@/services/audio";
import { createRipple } from "@/stores/ripple";
import { displayVolume, setVolume } from "@/stores/volume";
import { isHovering, setWidgetState } from "@/stores/widget";
import { setMedia } from "@/stores/media";
import { MediaService } from "@/services/media";

let timer: number | undefined;

export function useAudio() {
  onMount(async () => {
    setVolume({
      volume: await AudioService.getVolume(),
      muted: await AudioService.isMuted(),
    });

    const media = await MediaService.currentMediaInfo();
    setMedia(media);

    // await startAudioListener();
  });

  async function wheel(e: WheelEvent) {
    e.preventDefault();
    createRipple();
    displayVolume();
    setWidgetState("volume");
    clearTimeout(timer);

    timer = window.setTimeout(() => {
      setWidgetState(isHovering() ? "hover" : "idle");
    }, 700);

    if (e.deltaY < 0) {
      await AudioService.volumeUp();
    } else {
      await AudioService.volumeDown();
    }

    setVolume({
      volume: await AudioService.getVolume(),
      muted: await AudioService.isMuted(),
    });
  }

  async function toggleMute() {
    await AudioService.toggleMute();
    hideVolume();
    setVolume({
      volume: await AudioService.getVolume(),
      muted: await AudioService.isMuted(),
    });
  }

  return {
    volume,
    wheel,
    toggleMute,
  };
}
