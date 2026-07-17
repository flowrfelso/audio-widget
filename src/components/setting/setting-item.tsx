import ToggleButton from "@/components/common/button/toggle-button";

interface Props {
  title: string;
  checked: () => boolean;
  onChange: () => void;
}

export default function SettingItem(props: Props) {
  return (
    <div
      class="
        flex
        items-center
        justify-between
        py-4
        border-b
        border-zinc-700
        last:border-0
      "
    >
      <span class="text-white">{props.title}</span>

      <ToggleButton checked={props.checked()} onChange={props.onChange} />
    </div>
  );
}
