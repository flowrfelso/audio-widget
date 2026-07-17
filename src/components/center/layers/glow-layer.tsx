import Glow from "@/components/center/glow";

export default function GlowLayer() {
  return (
    <div
      class="
            absolute
            inset-0

            flex
            items-center
            justify-center

            z-0
            "
    >
      <Glow />
    </div>
  );
}
