import { createSignal } from "solid-js";

export interface Ripple {
  id: number;
}

export const [ripples, setRipples] = createSignal<Ripple[]>([]);

let id = 0;

export function createRipple() {
  const current = id++;

  setRipples((r) => [
    ...r,

    {
      id: current,
    },
  ]);

  window.setTimeout(() => {
    setRipples((r) => r.filter((x) => x.id !== current));
  }, 600);
}
