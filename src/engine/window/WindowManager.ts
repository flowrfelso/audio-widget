import { getCurrentWindow } from "@tauri-apps/api/window";

export class WindowManager {
  private readonly appWindow = getCurrentWindow();

  async position() {
    return this.appWindow.outerPosition();
  }

  async size() {
    return this.appWindow.outerSize();
  }

  //   async move(x: number, y: number) {
  //     await this.appWindow.setPosition({
  //       x,
  //       y,
  //     });
  //   }

  async show() {
    await this.appWindow.show();
  }

  async hide() {
    await this.appWindow.hide();
  }

  async alwaysOnTop(value: boolean) {
    await this.appWindow.setAlwaysOnTop(value);
  }
}

export const windowManager = new WindowManager();
