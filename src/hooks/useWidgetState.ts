import { setWidgetState } from "@/stores/widget";

let isHovering = false;

export function useWidgetState() {
  const enter = () => {
    setWidgetState("hover");
    isHovering = true;
  };

  const leave = () => {
    setWidgetState("idle");
    isHovering = false;
  };

  const dragging = () => {
    setWidgetState("dragging");
  };

  const volume = () => {
    setWidgetState("volume");
  };

  return {
    enter,

    leave,

    dragging,

    volume,
  };
}
