import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/checkbox";

export default {
    title: "Core / Inputs"
}

interface Args {
    label: string,
    error: string,
}

const DEFAULT_ARGS:Args = {
    label: "hello",
    error: ""
}

export const Checkbox = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {} = props

    return `<input-checkbox ${argsToAttrs(props)}></input-checkbox>`;
}

Checkbox.args = DEFAULT_ARGS;