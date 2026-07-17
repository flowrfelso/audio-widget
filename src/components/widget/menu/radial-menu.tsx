import { For } from "solid-js";
import { expanded } from "@/stores/widget";
import { MENU_RADIUS, menuItems } from "@/utils/menu";
import { polar } from "@/utils/polar";
import RadialButton from "@/components/widget/menu/radial-button";

export default function RadialMenu() {
  return (
    <>
      <For each={menuItems}>
        {(item) => {
          const pos = polar(MENU_RADIUS, item.angle);
          return (
            <RadialButton
              icon={item.icon}
              expanded={expanded()}
              x={pos.x}
              y={pos.y}
              onClick={item.action}
              isDynamic={item.isDynamic ?? false}
            />
          );
        }}
      </For>
    </>
  );
}
