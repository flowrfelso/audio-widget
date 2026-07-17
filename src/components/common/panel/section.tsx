import { JSX } from "solid-js";

interface Props {
  title?: string;
  children: JSX.Element;
}

export default function Section(props: Props) {
  return (
    <section class="space-y-4">
      {props.title && (
        <h2 class="text-sm font-semibold text-zinc-400 uppercase tracking-wide">
          {props.title}
        </h2>
      )}

      {props.children}
    </section>
  );
}
