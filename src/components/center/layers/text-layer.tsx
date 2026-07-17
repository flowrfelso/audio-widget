import VolumeText from "@/components/center/volume-text";

export default function TextLayer() {
  return (
    <div
      class="
            absolute
            inset-0

            flex
            items-center
            justify-center

            z-50
            "
    >
      <VolumeText />
    </div>
  );
}
