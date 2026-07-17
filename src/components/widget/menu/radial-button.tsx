import { Dynamic } from "solid-js/web";
import { Motion } from "solid-motionone";
interface RadialButton {
  x: number;
  y: number;
  expanded: boolean;
  isDynamic: boolean;
  icon: any;
  disabled?: boolean;
  onClick: () => void | Promise<void> | Promise<unknown>;
}
export default function RadialButton(props: RadialButton) {
  return (
    <Motion.div
      initial={{ x: 0, y: 0, opacity: 0, scale: 0.2 }}
      animate={{
        x: props.expanded ? props.x : 0,
        y: props.expanded ? props.y : 0,
        opacity: props.expanded ? 1 : 0,
        scale: props.expanded ? 1 : 0.2,
      }}
      transition={{ duration: 0.28, easing: [0.22, 1, 0.36, 1] }}
      class={`absolute w-12 h-12 rounded-full bg-zinc-900 border border-zinc-700 shadow-xl 
        flex items-center justify-center text-white 
        ${
          props.disabled
            ? "opacity-50 cursor-not-allowed"
            : "cursor-pointer hover:bg-zinc-800"
        }`}
      onClick={() => {
        if (!props.disabled) props.onClick();
      }}
    >
      {typeof props.icon === "function" && props.isDynamic ? (
        <Dynamic component={props.icon()} size={18} />
      ) : (
        <props.icon size={18} />
      )}
    </Motion.div>
  );
}
