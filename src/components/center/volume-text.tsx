import { showVolume, volume } from "@/stores/volume";
import { Motion, Presence } from "solid-motionone";

export default function VolumeText() {
  return (
    <Presence>
      {showVolume() && (
        <Motion.div
          initial={{
            opacity: 0,

            scale: 0.6,
          }}
          animate={{
            opacity: 1,

            scale: 1,
          }}
          exit={{
            opacity: 0,

            scale: 0.6,
          }}
          transition={{
            duration: 0.18,
          }}
          class="
                        absolute

                        z-40

                        text-white

                        font-semibold

                        text-lg

                        select-none
                        "
        >
          {volume().volume}%
        </Motion.div>
      )}
    </Presence>
  );
}
