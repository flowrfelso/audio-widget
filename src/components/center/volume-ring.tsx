import { Motion } from "solid-motionone";
import { volume } from "@/stores/volume";

const SIZE = 96;
const STROKE = 4;
const RADIUS = (SIZE - STROKE) / 2;

const CIRCUMFERENCE = 2 * Math.PI * RADIUS;

export default function VolumeRing() {
  const offset = () => CIRCUMFERENCE - (volume().volume / 100) * CIRCUMFERENCE;

  return (
    <Motion.svg
      width={SIZE}
      height={SIZE}
      viewBox={`0 0 ${SIZE} ${SIZE}`}
      class="absolute z-10 overflow-visible"
    >
      <circle
        cx={SIZE / 2}
        cy={SIZE / 2}
        r={RADIUS}
        fill="transparent"
        stroke="#3f3f46"
        stroke-width={STROKE}
      />

      <Motion.circle
        cx={SIZE / 2}
        cy={SIZE / 2}
        r={RADIUS}
        fill="transparent"
        stroke="#ffffff"
        stroke-width={STROKE}
        stroke-linecap="round"
        transform={`rotate(-90 ${SIZE / 2} ${SIZE / 2})`}
        stroke-dasharray={CIRCUMFERENCE.toString()}
        animate={{
          strokeDashoffset: offset(),
        }}
        transition={{
          duration: 0.2,
        }}
      />
    </Motion.svg>
  );
}
