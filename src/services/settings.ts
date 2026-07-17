import { invoke } from "@tauri-apps/api/core";

export const SettingsService = {
  get: () =>
    invoke<{
      always_on_top: boolean;
      launch_at_startup: boolean;
    }>("get_settings"),

  update: (key: string, value: boolean) =>
    invoke("update_setting", {
      key,
      value,
    }),
};
