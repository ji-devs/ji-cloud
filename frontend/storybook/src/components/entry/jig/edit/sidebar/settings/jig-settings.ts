import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-settings";
import "@elements/entry/jig/edit/sidebar/settings/jig-settings-button";
import "@elements/entry/jig/edit/sidebar/settings/jig-settings-switch-container";
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

export const JigSettings = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-settings ${argsToAttrs(props)}>
            <jig-settings-switch-container slot="settings">
                <input-switch slot="switch"></input-switch>
                <span slot="label">JIG direction: <span style="font-weight: 600">Left to right</span></span>
            </jig-settings-switch-container>
            <jig-settings-switch-container slot="settings">
                <input-switch slot="switch"></input-switch>
                <span slot="label">Display score</span>
            </jig-settings-switch-container>
            <jig-settings-button slot="settings" kind="background"></jig-settings-button>
            <jig-settings-button slot="settings" kind="feedback"></jig-settings-button>
        </jig-settings>
    `;
}
JigSettings.args = DEFAULT_ARGS;
