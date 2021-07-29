import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/play/move-button";
import { Kind } from "@elements/entry/jig/play/move-button";

export default {
    title: "Entry / Jig / Play"
}

interface Args {
    kind: Kind
}

const DEFAULT_ARGS:Args = {
    kind: "back",
}


export const MoveButton = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-play-move-button ${argsToAttrs(props)}></jig-play-move-button>
    `;
}
MoveButton.args = DEFAULT_ARGS;
MoveButton.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ["back", "froward"],
        }
    },
}
