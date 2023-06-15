import { argsToAttrs } from "@utils/attributes";
import "@elements/core/buttons/rectangle";
import { Color, Size, Kind } from "@elements/core/buttons/rectangle";

export default {
    title: "Core / Buttons",
};
interface Args {
    size: Size;
    color: Color;
    kind: Kind;
    submit: boolean;
    disabled: boolean;
    href: string;
    target: string;

    contents: string;
}

const DEFAULT_ARGS: Args = {
    size: "regular",
    color: "red",
    kind: "filled",
    submit: false,
    disabled: false,
    href: "",
    target: "",

    contents: "Submit",
};

export const Rectangle = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `<button-rect ${argsToAttrs(props)}>${props.contents}</button-rect>`;
};

Rectangle.args = DEFAULT_ARGS;

Rectangle.argTypes = {
    size: {
        control: {
            type: "inline-radio",
            options: ["regular", "large"],
        },
    },
    color: {
        control: {
            type: "inline-radio",
            options: ["red", "blue", "green", "grey"],
        },
    },
    kind: {
        control: {
            type: "inline-radio",
            options: ["filled", "text", "outline"],
        },
    },
};
