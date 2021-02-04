import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/dropdowns/dropdown-underlined";

export default {
    title: "Core/Inputs"
}

interface Args {
    closed:boolean
}

const DEFAULT_ARGS:Args = {
    closed:false
}

export const DropdownUnderlined = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<dropdown-underlined ${argsToAttrs(props)}></dropdown-underlined ${closed && "closed"}>`;
}

DropdownUnderlined.args = DEFAULT_ARGS;