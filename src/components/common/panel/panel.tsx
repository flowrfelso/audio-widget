import { JSX } from "solid-js";

interface Props {
  children: JSX.Element;
  class?: string;
}

export default function Panel(props: Props) {
  return (
    <div
      class={`
        border
        border-zinc-700
        bg-[#2b2b2b]
        shadow-2xl
        ${props.class ?? ""}
      `}
    >
      {props.children}
    </div>
  );
}
