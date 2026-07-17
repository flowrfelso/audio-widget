import DragLayer from "./DragLayer";
import RadialMenu from "@/components/widget/menu/radial-menu";
// import { useVolumeWheel } from "@/hooks/useVolumeWheel";
import Center from "@/components/center/center";
import { useWidgetState } from "@/hooks/useWidgetState";
// import { useVolumeWheel } from "@/hooks/useVolumeWheel";
import { useAudio } from "@/hooks/useAudio";

export default function FloatingWidget() {
  // const { onWheel } = useVolumeWheel();
  const widget = useWidgetState();
  const audio = useAudio();

  return (
    <div
      class="
            relative
            w-65
            h-65
            flex
            items-center
            justify-center
            "
      onWheel={audio.wheel}
      onMouseEnter={widget.enter}
      onMouseLeave={widget.leave}
    >
      <DragLayer />
      <RadialMenu />
      <Center />
    </div>
  );
}
