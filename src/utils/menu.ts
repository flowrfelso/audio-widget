import { MediaService } from "@/services/media";
import { media } from "@/stores/media";
import { MenuItem } from "@/types/widget";
import {
  Play,
  Pause,
  SkipBack,
  SkipForward,
  ChevronsLeft,
  ChevronsRight,
  StepBack,
  // StepForward,
} from "lucide-solid";
export const MENU_RADIUS = 90;

export const menuItems: MenuItem[] = [
  {
    id: "seek-back-30",
    icon: StepBack,
    angle: 90,
    action: MediaService.seekBack30,
  },
  {
    id: "previous",
    icon: SkipBack,
    angle: 150,
    action: MediaService.previous,
    disabled: !media.canPrevious,
  },
  {
    id: "seek-back-10",
    icon: ChevronsLeft,
    angle: 210,
    action: MediaService.seekBack10,
  },
  // {
  //   id: "seek-forward-30",
  //   icon: StepForward,
  //   angle: -90,
  //   action: MediaService.seekBack30,
  // },
  {
    id: "next",
    icon: SkipForward,
    angle: -30,
    action: MediaService.next,
    disabled: !media.canNext,
  },
  {
    id: "seek-forward-10",
    icon: ChevronsRight,
    angle: 30,
    action: MediaService.seekForward10,
  },
  {
    id: "play-pause",
    icon: () => (media.playing ? Pause : Play),
    angle: 270,
    action: () => {
      MediaService.togglePlayPause();
    },
    isDynamic: true,
  },
];
