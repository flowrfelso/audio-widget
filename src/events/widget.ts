import { createSignal } from "solid-js";

export type WidgetEvent =
  | "volume-up"
  | "volume-down"
  | "mute"
  | "next"
  | "previous"
  | "play"
  | "pause";

export const [widgetEvent, setWidgetEvent] = createSignal<WidgetEvent>();
