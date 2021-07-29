import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-settings";
import "@elements/entry/jig/edit/sidebar/settings/jig-settings-button";
import { JigPreviewSettings } from "./jig-preview-settings";

export default {
    title: "Entry / Jig / Edit / Sidebar / Settings"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const JigSettings = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-settings ${argsToAttrs(props)}>
            <jig-settings-button slot="creator" kind="theme"></jig-settings-button>
            <jig-settings-button slot="creator" kind="background"></jig-settings-button>
            <jig-settings-button slot="creator" kind="feedback"></jig-settings-button>
            ${JigPreviewSettings()}
        </jig-settings>
    `;
}
JigSettings.args = DEFAULT_ARGS;
