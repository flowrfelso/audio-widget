import { For } from "solid-js";

import { Motion, Presence } from "solid-motionone";

import { ripples } from "@/stores/ripple";

export default function Ripple() {
  return (
    <Presence>
      <For each={ripples()}>
        {(r) => (
          <Motion.div
            initial={{
              scale: 0.6,

              opacity: 0.35,
            }}
            animate={{
              scale: 2.4,

              opacity: 0,
            }}
            exit={{
              opacity: 0,
            }}
            transition={{
              duration: 0.6,

              easing: "ease-out",
            }}
            class="
                        absolute
                        w-20
                        h-20
                        rounded-full
                        border
                        border-white/60

                        "
          />
        )}
      </For>
    </Presence>
  );
}
