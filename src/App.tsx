import { useAudioService } from "@/services/audio.service";

import FloatingWidget from "@/components/FloatingWidget";
import { useWindowEvents } from "@/hooks/useWindowEvent";
import { onCleanup, onMount } from "solid-js";
import { startAudioListener } from "@/lib/tauri/audio";

function App() {
  // useAudioService;
  useWindowEvents;

  let unlisten: (() => void) | undefined;

  // onMount(async () => {
  //   unlisten = await startAudioListener();
  // });

  onCleanup(() => {
    unlisten?.();
  });

  return <FloatingWidget />;
}

export default App;
