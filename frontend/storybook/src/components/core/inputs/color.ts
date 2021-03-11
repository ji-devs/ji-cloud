import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/color";
import "@elements/core/buttons/rectangle";

export default {
    title: "Core / Inputs"
}

interface Args {
}

const DEFAULT_ARGS: Args = {
}

export const Color = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <input-color ${argsToAttrs(props)}>
            <button-rect>Select color</button-rect>
        </input-color>
    `;
}

Color.args = DEFAULT_ARGS;
