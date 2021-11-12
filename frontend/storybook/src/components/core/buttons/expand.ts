import { argsToAttrs } from "@utils/attributes";
import "@elements/core/buttons/expand";

export default {
    title: "Core / Buttons",
};

interface Args {
    expanded: boolean;
}

const DEFAULT_ARGS: Args = {
    expanded: false,
};

export const Expand = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `<button-expand ${argsToAttrs(props)}></button-expand>`;
};

Expand.args = DEFAULT_ARGS;
