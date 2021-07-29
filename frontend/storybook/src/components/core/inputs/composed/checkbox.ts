import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/composed/checkbox";

export default {
    title: "Core / Inputs / Composed"
}

interface Args {
    label: string,
    error: string,
    checked: boolean,
}

const DEFAULT_ARGS:Args = {
    label: "hello",
    error: "",
    checked: false,
}

export const Checkbox = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {} = props

    return `<input-checkbox ${argsToAttrs(props)}></input-checkbox>`;
}

Checkbox.args = DEFAULT_ARGS;