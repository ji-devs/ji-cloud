import { argsToAttrs } from "@utils/attributes";
import "@elements/core/buttons/main";
import { Color, Size, Kind } from "@elements/core/buttons/main";

export default {
    title: "Core / Buttons",
};
interface Args {
    size: Size;
    color: Color;
    kind: Kind;
    bold: boolean;
    italic: boolean;
    submit: boolean;
    disabled: boolean;

    contents: string;
}

const DEFAULT_ARGS: Args = {
    size: "medium",
    color: "red",
    kind: "rect",
    bold: false,
    italic: false,
    submit: false,
    disabled: false,

    contents: "Submit",
};

export const Main = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `<button-main ${argsToAttrs(props)}>${props.contents}</button-main>`;
};

Main.args = DEFAULT_ARGS;

Main.argTypes = {
    size: {
        control: {
            type: "inline-radio",
            options: ["small", "medium", "large", "x-large"],
        },
    },
    color: {
        control: {
            type: "inline-radio",
            options: ["grey", "red", "blue", "green"],
        },
    },
    kind: {
        control: {
            type: "inline-radio",
            options: ["rect", "text", "outline"],
        },
    },
};
