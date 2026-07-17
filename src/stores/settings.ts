import { createStore } from "solid-js/store";
import { SettingKey } from "@/types/settings";

export const [settings, setSettings] = createStore<Record<SettingKey, boolean>>(
  {
    alwaysOnTop: false,
    launchAtStartup: false,
  }
);
