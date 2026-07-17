import { displayVolume, setVolume } from "@/stores/volume";

import { createRipple } from "@/stores/ripple";

import { AudioService } from "@/services/audio";
import { onMount } from "solid-js";
import { setWidgetState } from "@/stores/widget";

const STEP = 2;

let timer: number | undefined;

export function useVolumeWheel() {
  onMount(async () => {
    setVolume({
      volume: await AudioService.getVolume(),
      muted: await AudioService.isMuted(),
    });

    // await startAudioListener();
  });

  const onWheel = async (event: WheelEvent) => {
    event.preventDefault();
    createRipple();
    displayVolume();
    setWidgetState("volume");
    clearTimeout(timer);
    timer = window.setTimeout(() => {
      setWidgetState("hover");
    }, 700);

    if (event.deltaY < 0) {
      await AudioService.volumeUp(STEP);
    } else {
      await AudioService.volumeDown(STEP);
    }
  };

  return {
    onWheel,
  };
}
