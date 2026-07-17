import { createEffect } from "solid-js";

import { invoke } from "@tauri-apps/api/core";

import { widgetEvent } from "../events/widget";

export function useAudioService() {
  createEffect(async () => {
    const event = widgetEvent();

    if (!event) return;

    switch (event) {
      case "volume-up":
        await invoke("volume_up");

        break;

      case "volume-down":
        await invoke("volume_down");

        break;

      case "mute":
        await invoke("toggle_mute");

        break;
    }
  });
}
