import { SettingKey } from "@/types/settings";

export const SETTINGS = [
  {
    group: "General",
    items: [
      {
        key: "alwaysOnTop" as SettingKey,
        title: "Always on Top",
      },
      {
        key: "launchAtStartup" as SettingKey,
        title: "Launch at Startup",
      },
    ],
  },
];
