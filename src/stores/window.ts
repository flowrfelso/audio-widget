import { getCurrentWindow } from "@tauri-apps/api/window";
import { createStore } from "solid-js/store";

export interface WindowPosition {
  x: number;
  y: number;
}

export interface WindowState extends WindowPosition {
  isDragging: boolean;
  isSnapped: boolean;
  isPinned: boolean;
}

const appWindow = getCurrentWindow();

const [state, setState] = createStore<WindowState>({
  x: 0,
  y: 0,
  isDragging: false,
  isSnapped: false,
  isPinned: false,
});

export const WindowService = {
  get state() {
    return state;
  },

  setPosition: (x: number, y: number) => setState({ x, y }),
  setDragging: (dragging: boolean) => setState("isDragging", dragging),
  setSnapped: (snapped: boolean) => setState("isSnapped", snapped),

  togglePin: async () => {
    const newState = !state.isPinned;
    await appWindow.setAlwaysOnTop(newState);
    setState("isPinned", newState);
  },
};
