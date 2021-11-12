import { argsToAttrs } from "@utils/attributes";
import "@elements/core/buttons/icon";
import { IconKind } from "@elements/core/buttons/icon";

export default {
    title: "Core / Buttons",
};

interface Args {
    icon: IconKind;
}

const DEFAULT_ARGS: Args = {
    icon: "gears",
};

export const Icon = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `<button-icon ${argsToAttrs(props)}></button-icon>`;
};

Icon.args = DEFAULT_ARGS;

Icon.argTypes = {
    icon: {
        control: {
            type: "inline-radio",
            options: [
                "circle-x-blue",
                "circle-+-blue",
                "audio",
                "audio-stop",
                "white-circle-blue-arrow",
                "circle-check",
                "circle-kebab-grey",
                "circle-kebab-blue",
                "circle-pencil",
                "gears",
                "x",
            ],
        },
    },
};
