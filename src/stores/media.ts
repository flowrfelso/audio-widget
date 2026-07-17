import { MediaInfo } from "@/types/media";
import { createStore } from "solid-js/store";

const [media, setMedia] = createStore<MediaInfo>({
  title: "",
  artist: "",
  source: "",
  playing: false,
  canNext: false,
  canPrevious: false,
});

export { media, setMedia };
