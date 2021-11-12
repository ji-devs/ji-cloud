import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/composed/search";

export default {
    title: "Core / Inputs / Composed",
};

interface Args {
    placeholder: string;
    value: string;
}

const DEFAULT_ARGS: Args = {
    placeholder: "hello",
    value: "world",
};

export const Search = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    const {} = props;

    return `<input-search ${argsToAttrs(props)}></input-search>`;
};

Search.args = DEFAULT_ARGS;
