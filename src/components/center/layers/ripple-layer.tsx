import Ripple from "@/components/center/layers/ripple";

export default function RippleLayer() {
  return (
    <div
      class="
            absolute
            inset-0
            flex
            items-center
            justify-center
            z-10
            pointer-events-none
            "
    >
      <Ripple />
    </div>
  );
}
