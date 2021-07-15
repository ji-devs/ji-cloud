import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_groups/cards/play/card/empty";
import {Kind} from "@elements/module/_groups/cards/play/card/empty";
import {mapToString, arrayIndex} from "@utils/array";
import {ThemeId, ThemeControl} from "~/components/module/_common/theme";
import {Ji as MockJiImage} from "~/components/core/images/ji";
import {Size} from "@elements/module/_groups/cards/play/card/styles";

export default {
    title: "Module / _GROUPS / Cards / play"
}

export interface Args {
    theme: ThemeId,
    size: Size,
    kind: Kind,
    active?: boolean,
    slot?: string,
}

const DEFAULT_ARGS:Args = {
    theme: "chalkboard",
    size: "memory",
    kind: "question",
}

export const Empty = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
    <empty-card ${argsToAttrs(props)} >
    </empty-card>`;
}


Empty.args = DEFAULT_ARGS;
Empty.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ["question", "translucent"]
        }
    },
    theme: ThemeControl
}
