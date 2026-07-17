// import { createSignal } from "solid-js";

// export const [expanded, setExpanded] = createSignal(false);

import { createMemo } from "solid-js";
import { createStore } from "solid-js/store";

interface WidgetStore {
  state: "idle" | "hover" | "dragging" | "volume" | "hidden";
  isHovering: boolean;
}

const [store, setStore] = createStore<WidgetStore>({
  state: "idle",
  isHovering: false,
});

const widgetState = () => store.state;
const isHovering = () => store.isHovering;
const isExpanded = () => store.state === "hover" || store.state === "volume";

// --- Export các hành động (Actions) ---
const setWidgetState = (newState: WidgetStore["state"]) =>
  setStore("state", newState);
const setHovering = (val: boolean) => setStore("isHovering", val);

const expanded = createMemo(() => {
  return store.state === "hover" || store.state === "volume";
});

export {
  widgetState,
  isHovering,
  isExpanded,
  setWidgetState,
  setHovering,
  expanded,
};
