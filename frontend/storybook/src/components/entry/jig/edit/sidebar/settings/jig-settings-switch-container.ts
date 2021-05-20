import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-settings-switch-container";

export default {
    title: "Entry / Jig / Edit / Sidebar / Settings"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const SettingsSwitchContainer = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-settings-switch-container ${argsToAttrs(props)}>
            <input-switch slot="switch"></input-switch>
            <span slot="label">JIG direction: Left to right</span>
        </jig-settings-switch-container>
    `;
}
SettingsSwitchContainer.args = DEFAULT_ARGS;
