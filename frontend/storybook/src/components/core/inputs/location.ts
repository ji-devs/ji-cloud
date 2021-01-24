import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/location";

export default {
    title: "Core / Inputs"
}

interface Args {
    error: string,
    value: string,
    help: string,
    placeholder: string,
}

const DEFAULT_ARGS:Args = {
    error: "",
    value: "",
    help: "",
    placeholder: "",
}

export const Location = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<input-location ${argsToAttrs(props)} ></input-location>`
}

Location.args = DEFAULT_ARGS;