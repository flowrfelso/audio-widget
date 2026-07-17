import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

export async function startDrag() {
  await appWindow.startDragging();
}
