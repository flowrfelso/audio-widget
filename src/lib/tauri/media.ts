import { invoke } from "@tauri-apps/api/core";

export function playPause() {
  return invoke("media_play_pause");
}

export function nextTrack() {
  return invoke("media_next");
}

export function previousTrack() {
  return invoke("media_previous");
}

export function seekForward(seconds: number) {
  return invoke("media_seek_forward", {
    seconds,
  });
}

export function seekBackward(seconds: number) {
  return invoke("media_seek_backward", {
    seconds,
  });
}
