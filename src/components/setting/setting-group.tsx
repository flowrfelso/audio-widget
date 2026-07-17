import Section from "@/components/common/panel/section";
import { JSX } from "solid-js";

interface Props {
  title: string;
  children: JSX.Element;
}

export default function SettingGroup(props: Props) {
  return (
    <Section title={props.title}>
      <div>{props.children}</div>
    </Section>
  );
}
