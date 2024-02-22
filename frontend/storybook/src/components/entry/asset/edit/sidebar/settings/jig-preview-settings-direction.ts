import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/composed/switch-direction";
import { Direction } from "@elements/core/inputs/composed/switch-direction";

export default {
    title: "Entry / Jig / Edit / Sidebar / Settings",
};

interface Args {
    direction: Direction;
}

const DEFAULT_ARGS: Args = {
    direction: "ltr",
};

export const PreviewDirection = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <input-switch-direction ${argsToAttrs(
            props
        )}></input-switch-direction>
    `;
};
PreviewDirection.args = DEFAULT_ARGS;
