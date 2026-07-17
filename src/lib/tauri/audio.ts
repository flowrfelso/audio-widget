import { invoke } from "@tauri-apps/api/core";

import { listen } from "@tauri-apps/api/event";

import { setAudio } from "@/stores/audio";

export async function startAudioListener() {
  return listen<{
    volume: number;

    muted: boolean;
  }>("audio://changed", ({ payload }) => {
    setAudio({
      volume: payload.volume,

      muted: payload.muted,
    });
  });
}

export function volumeUp(step = 2) {
  return invoke("volume_up", {
    step,
  });
}

export function volumeDown(step = 2) {
  return invoke("volume_down", {
    step,
  });
}

export function setVolume(volume: number) {
  return invoke("set_volume", {
    volume,
  });
}

export function toggleMute() {
  return invoke("toggle_mute");
}

export function getVolume() {
  return invoke<number>("get_volume");
}
