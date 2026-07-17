import { invoke } from "@tauri-apps/api/core";

const STEP = 2;

export interface AudioState {
  volume: number;
  muted: boolean;
}

export const AudioService = {
  getVolume() {
    return invoke<number>("get_volume");
  },

  setVolume(volume: number) {
    return invoke<void>("set_volume", {
      volume,
    });
  },

  volumeUp(step = STEP) {
    return invoke<void>("volume_up", {
      step,
    });
  },

  volumeDown(step = STEP) {
    return invoke<void>("volume_down", {
      step,
    });
  },

  toggleMute() {
    return invoke<void>("toggle_mute");
  },

  isMuted() {
    return invoke<boolean>("is_muted");
  },
};
