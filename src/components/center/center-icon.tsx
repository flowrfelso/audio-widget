import { Motion, Presence } from "solid-motionone";
import { Volume2, VolumeX } from "lucide-solid";
import { expanded } from "@/stores/widget";
import { showVolume, volume } from "@/stores/volume";

export default function CenterIcon() {
  return (
    <Presence>
      {!showVolume() && (
        <Motion.div
          initial={{
            opacity: 0,

            scale: 0.6,
          }}
          animate={{
            opacity: 1,

            scale: expanded() ? 1.08 : 1,
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
                        z-30
                        w-20
                        h-20
                        rounded-full
                        bg-zinc-900
                        border
                        border-zinc-700
                        shadow-2xl
                        flex
                        items-center
                        justify-center
                        text-white
                        "
        >
          {volume().muted ? <VolumeX size={28} /> : <Volume2 size={28} />}
        </Motion.div>
      )}
    </Presence>
  );
}
