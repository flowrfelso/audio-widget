import { onCleanup, onMount } from "solid-js";

import { settings, setSettings } from "@/stores/settings";
import { SettingsService } from "@/services/settings";
import { SettingKey } from "@/types/settings";
import { listen } from "@tauri-apps/api/event";

export function useSettings() {
  onMount(async () => {
    const data = await SettingsService.get();

    setSettings({
      alwaysOnTop: data.always_on_top,
      launchAtStartup: data.launch_at_startup,
    });

    const unlisten = await listen<[SettingKey, boolean]>(
      "setting-changed",
      (event) => {
        const [key, value] = event.payload;
        setSettings(key, value);
        console.log("🚀 ~ useSettings ~ setting-changed:");
      }
    );

    // Trong onCleanup:
    onCleanup(() => {
      unlisten();
    });
  });

  async function toggle(key: SettingKey) {
    const newValue = !settings[key];
    await SettingsService.update(key, newValue);
    setSettings(key, newValue);
  }

  return {
    settings,

    toggle,
  };
}
