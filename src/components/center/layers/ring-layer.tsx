import VolumeRing from "@/components/center/volume-ring";

export default function RingLayer() {
  return (
    <div
      class="
            absolute
            inset-0
            flex
            items-center
            justify-center
            z-20
            "
    >
      <VolumeRing />
    </div>
  );
}
