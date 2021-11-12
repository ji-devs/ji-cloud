import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/old/location";
import "@elements/core/inputs/primitives/location";

export default {
    title: "Core / Inputs / Primitives",
};

interface Args {
    value: string;
    placeholder: string;
}

const DEFAULT_ARGS: Args = {
    value: "",
    placeholder: "",
};

export const Location = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `<input-location ${argsToAttrs(props)} ></input-location>`;
};

Location.args = DEFAULT_ARGS;
