import Panel from "@/components/common/panel/panel";
import SettingGroup from "@/components/setting/setting-group";
import SettingItem from "@/components/setting/setting-item";
import SettingsLayout from "@/components/setting/settings-layout";
import { useSettings } from "@/hooks/useSettings";
import { SETTINGS } from "@/utils/settings";
import { For } from "solid-js";
import { settings } from "@/stores/settings";

export default function SettingsView() {
  const { toggle } = useSettings();

  return (
    <div
      class="
        h-screen
        w-screen
        flex
        items-center
        justify-center
      "
    >
      <Panel>
        <SettingsLayout>
          <For each={SETTINGS}>
            {(group) => (
              <SettingGroup title={group.group}>
                <For each={group.items}>
                  {(item) => (
                    <SettingItem
                      title={item.title}
                      checked={() => settings[item.key]}
                      onChange={() => toggle(item.key)}
                    />
                  )}
                </For>
              </SettingGroup>
            )}
          </For>
        </SettingsLayout>
      </Panel>
    </div>
  );
}
