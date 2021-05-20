import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-settings-button";
import {Kind} from "@elements/entry/jig/edit/sidebar/settings/jig-settings-button";

export default {
    title: "Entry / Jig / Edit / Sidebar / Settings"
}

interface Args {
    kind: Kind,
}

const DEFAULT_ARGS:Args = {
    kind: "feedback"
}

export const SettingsButton = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-settings-button ${argsToAttrs(props)}></jig-settings-button>
    `;
}
SettingsButton.args = DEFAULT_ARGS;
SettingsButton.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ["background", "effects"],
        }
    },
}
