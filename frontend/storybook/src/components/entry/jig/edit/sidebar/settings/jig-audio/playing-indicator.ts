import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-audio/jig-audio-playing-indicator";

export default {
    title: "Entry / Jig / Edit / Sidebar/ Settings/ Jig Audio"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const PlayingIndicator = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-audio-playing-indicator ${argsToAttrs(props)}></jig-audio-playing-indicator>
    `;
}
PlayingIndicator.args = DEFAULT_ARGS;
