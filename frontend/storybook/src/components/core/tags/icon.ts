import { argsToAttrs } from "@utils/attributes";
import "@elements/core/tags/icon";
import { IconKind } from "@elements/core/tags/icon";

export default {
    title: "Core / Tags",
};

interface Args {
    kind: IconKind;
    label: string;
}

const DEFAULT_ARGS: Args = {
    kind: "age",
    label: "hello",
};

export const Icon = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;
    const {} = props;

    return `<tag-icon ${argsToAttrs(props)}></tag-icon>`;
};

Icon.args = DEFAULT_ARGS;

Icon.argTypes = {
    kind: {
        control: {
            type: "inline-radio",
            options: ["age", "lang"],
        },
    },
};
