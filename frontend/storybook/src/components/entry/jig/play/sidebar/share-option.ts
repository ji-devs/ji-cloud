import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/play/sidebar/share-option";
import { Kind } from "@elements/entry/jig/play/sidebar/share-option";

export default {
    title: "Entry / Jig / Play / Sidebar"
}

interface Args {
    kind: Kind,
}

const DEFAULT_ARGS:Args = {
    kind: "students",
}


export const ShareOption = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-play-sidebar-share-option ${argsToAttrs(props)}></jig-play-sidebar-share-option>
    `;
}
ShareOption.args = DEFAULT_ARGS;
ShareOption.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ["students", "embed", "copy"],
        }
    },
}
