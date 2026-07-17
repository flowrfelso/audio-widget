import { onMount } from "solid-js";

import { setWindowPosition } from "../stores/window";

import { windowManager } from "../engine/window";

export function useWindowEngine() {
  onMount(async () => {
    const pos = await windowManager.position();

    setWindowPosition({
      x: pos.x,
      y: pos.y,
    });
  });
}
