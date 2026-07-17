import { MediaService } from "@/services/media";
import { setMedia } from "@/stores/media";
import { onMount } from "solid-js";

export function useMedia() {
  onMount(async () => {
    setMedia(await MediaService.currentMediaInfo());
  });

  return;
}
