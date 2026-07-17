import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

const DRAG_THRESHOLD = 5;

export function useDragClick(onClick: () => void) {
  let startX = 0;
  let startY = 0;

  let dragging = false;

  function mouseDown(e: MouseEvent) {
    if (e.button !== 0) return;

    dragging = false;

    startX = e.clientX;
    startY = e.clientY;
  }

  async function mouseMove(e: MouseEvent) {
    if (dragging) return;

    const dx = e.clientX - startX;
    const dy = e.clientY - startY;

    if (Math.hypot(dx, dy) > DRAG_THRESHOLD) {
      dragging = true;

      await appWindow.startDragging();
    }
  }

  function click() {
    if (!dragging) {
      onClick();
    }
  }

  return {
    mouseDown,
    mouseMove,
    click,
  };
}
