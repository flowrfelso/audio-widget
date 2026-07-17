interface Props {
  checked: boolean;
  onChange: () => void;
}

export default function ToggleButton(props: Props) {
  return (
    <button
      onClick={props.onChange}
      class={`
        flex
        items-center
        gap-2
        px-3
        py-1
        transition-all
        duration-200
        ${
          props.checked
            ? "border-[#4cc2ff] hover:border-[#33aadd]"
            : "border-[#cccccc] hover:border-[#aaaaaa]"
        }
      `}
    >
      <span
        class={`
          text-xs
          font-medium
          w-7
          text-right
          ${
            props.checked
              ? "text-[#4cc2ff] hover:border-[#33aadd]"
              : "text-[#cccccc] hover:border-[#aaaaaa]"
          }
        `}
      >
        {props.checked ? "On" : "Off"}
      </span>

      <div
        class={`
          relative
          w-10
          h-5
          rounded-full
          transition-all
          ${
            props.checked
              ? "bg-[#4cc2ff] hover:bg-[#33aadd]"
              : "bg-zinc-600 hover:bg-zinc-500"
          }
        `}
      >
        <div
          class={`
            absolute
            top-[1.5px]
            left-0.5
            h-4
            w-4
            rounded-full
            bg-white
            transition-transform
            ${props.checked ? "translate-x-5" : ""}
          `}
        />
      </div>
    </button>
  );
}
