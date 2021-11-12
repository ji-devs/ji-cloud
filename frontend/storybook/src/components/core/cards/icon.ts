import { argsToAttrs } from "@utils/attributes";
import "@elements/core/cards/icon";
import { IconKind } from "@elements/core/cards/icon";

export default {
    title: "Core / Cards",
};

interface Args {
    label: string;
    icon: IconKind;
}

const DEFAULT_ARGS: Args = {
    label: "hello",
    icon: "group",
};

export const Icon = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `<card-icon ${argsToAttrs(props)} />`;
};

Icon.args = DEFAULT_ARGS;

Icon.argTypes = {
    icon: {
        control: {
            type: "inline-radio",
            options: ["group"],
        },
    },
};
