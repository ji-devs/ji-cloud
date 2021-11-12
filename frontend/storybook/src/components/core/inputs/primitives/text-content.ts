import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/primitives/text-content";

export default {
    title: "Core / Inputs / Primitives",
};

interface Args {
    value: string;
    editing: boolean;
}

const DEFAULT_ARGS: Args = {
    value: "hello שָׁלוֹם",
    editing: false,
};

export const TextContent = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;
    const {} = props;

    return `<input-text-content ${argsToAttrs(props)}></input-text-content>`;
};

TextContent.args = DEFAULT_ARGS;
