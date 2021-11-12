import { argsToAttrs } from "@utils/attributes";
import "@elements/core/buttons/empty";

export default {
    title: "Core / Buttons",
};
interface Args {}

const DEFAULT_ARGS: Args = {};

export const Empty = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `<button-empty ${argsToAttrs(props)}>Button contents</button-empty>`;
};

Empty.args = DEFAULT_ARGS;
