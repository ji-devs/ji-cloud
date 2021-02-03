import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/dropdowns/dropdown-underlined";

export default {
    title: "Core/Inputs"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const DropdownUnderlined = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<dropdown-underlined ${argsToAttrs(props)}></dropdown-underlined>`;
}

DropdownUnderlined.args = DEFAULT_ARGS;