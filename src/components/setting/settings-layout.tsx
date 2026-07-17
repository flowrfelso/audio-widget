import { JSX } from "solid-js";

interface Props {
  children: JSX.Element;
}

export default function SettingsLayout(props: Props) {
  return (
    <div
      class="
        w-screen
        h-screen
        p-6
        space-y-8
      "
    >
      {props.children}
    </div>
  );
}
