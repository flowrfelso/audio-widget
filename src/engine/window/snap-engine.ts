import { getCurrentWindow } from "@tauri-apps/api/window";

const SNAP_DISTANCE = 24;

export class SnapEngine {
  async handleMoved() {
    const appWindow = getCurrentWindow();

    const pos = await appWindow.outerPosition();
    const size = await appWindow.outerSize();

    // Tạm thời chỉ log.
    // Bước sau sẽ tính work area và snap.

    console.log(pos, size);
  }
}

export const snapEngine = new SnapEngine();
