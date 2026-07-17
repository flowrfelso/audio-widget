import { createSignal } from "solid-js";

export interface AudioStore {
  volume: number;
  muted: boolean;
}

export const [volume, setVolume] = createSignal<AudioStore>({
  volume: 0,
  muted: false,
});

export const [showVolume, setShowVolume] = createSignal(false);

let timer: number | undefined;

export function displayVolume() {
  setShowVolume(true);

  if (timer) {
    clearTimeout(timer);
  }

  timer = window.setTimeout(() => {
    setShowVolume(false);
  }, 1000);
}

export function hideVolume() {
  setShowVolume(false);
}
