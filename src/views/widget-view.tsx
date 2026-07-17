import FloatingWidget from "@/components/FloatingWidget";

import { useAudioService } from "@/services/audio.service";
import { useWindowEvents } from "@/hooks/useWindowEvent";

import { onCleanup } from "solid-js";
import { startAudioListener } from "@/lib/tauri/audio";

export default function WidgetView() {
  useAudioService;
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
