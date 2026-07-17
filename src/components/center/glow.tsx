import { volume } from "@/stores/volume";
import { Motion } from "solid-motionone";

const baseOpacity = 0.2;
const baseScale = 0.9;

export default function Glow() {
  return (
    <Motion.div
      animate={{
        opacity: baseOpacity + volume().volume / 50,
        scale: baseScale + volume().volume / 150,
      }}
      transition={{
        duration: 0.2,
      }}
      class="
            absolute

            w-24
            h-24

            rounded-full

            bg-white/15

            blur-xl

            "
    />
  );
}
