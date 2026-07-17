import GlowLayer from "@/components/center/layers/glow-layer";
import IconLayer from "@/components/center/layers/icon-layer";
import RingLayer from "@/components/center/layers/ring-layer";
import RippleLayer from "@/components/center/layers/ripple-layer";
import TextLayer from "@/components/center/layers/text-layer";
import { useAudio } from "@/hooks/useAudio";
import { useDragClick } from "@/hooks/useDragClick";

export default function Center() {
  const audio = useAudio();
  const drag = useDragClick(audio.toggleMute);

  return (
    <div
      class="
            relative
            w-24
            h-24
            cursor-pointer
            "
      onClick={audio.toggleMute}
      onMouseDown={drag.mouseDown}
      onMouseMove={drag.mouseMove}
    >
      <GlowLayer />
      <RippleLayer />
      <RingLayer />
      <IconLayer />
      <TextLayer />
    </div>
  );
}
