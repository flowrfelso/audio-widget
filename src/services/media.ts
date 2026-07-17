import { setMedia } from "@/stores/media";
import { MediaInfo } from "@/types/media";
import { invoke } from "@tauri-apps/api/core";

const executeAndSyncMedia = async (command: string, args?: any) => {
  const result = await invoke<any>(command, args);
  const info = await MediaService.currentMediaInfo();
  setMedia(info);
  return result;
};

export const MediaService = {
  play: () => executeAndSyncMedia("play"),
  togglePlayPause: () => executeAndSyncMedia("toggle_play_pause"),
  previous: () => executeAndSyncMedia("previous"),
  next: () => executeAndSyncMedia("next"),
  //
  pause: () => invoke("pause"),
  seekForward10: () => invoke("seek_forward_10"),
  seekForward30: () => invoke("seek_forward_30"),
  seekBack10: () => invoke("seek_back_10"),
  seekBack30: () => invoke("seek_back_30"),
  currentMediaInfo: () => invoke<MediaInfo>("current_media_info"),
};
