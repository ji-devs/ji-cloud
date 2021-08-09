import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/primitives/select/base-option";

export default {
    title: "Core / Inputs / Primitives / Select"
}

interface Args {
    selected: boolean,

}

const DEFAULT_ARGS: Args = {
    selected: false,

}

export const BaseOption = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <base-option ${argsToAttrs(props)}></base-option>
    `;
}

BaseOption.args = DEFAULT_ARGS;
