import { onCleanup, onMount } from "solid-js";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { snapEngine } from "@/engine/window/snap-engine";

export function useWindowEvents() {
  onMount(async () => {
    const unlisten = await getCurrentWindow().onMoved(() => {
      snapEngine.handleMoved();
    });

    onCleanup(() => {
      unlisten();
    });
  });
}
