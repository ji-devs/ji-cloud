import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-preview-settings-direction";
import { Direction } from "@elements/entry/jig/edit/sidebar/settings/jig-preview-settings-direction";

export default {
    title: "Entry / Jig / Edit / Sidebar / Settings"
}

interface Args {
    direction: Direction,
}

const DEFAULT_ARGS:Args = {
    direction: "ltr"
}

export const PreviewDirection = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-preview-settings-direction ${argsToAttrs(props)}></jig-preview-settings-direction>
    `;
}
PreviewDirection.args = DEFAULT_ARGS;
