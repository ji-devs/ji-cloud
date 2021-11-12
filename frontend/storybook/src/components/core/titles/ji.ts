import { argsToAttrs } from "@utils/attributes";
import "@elements/core/titles/ji";
import { Color, Size } from "@elements/core/titles/ji";

export default {
    title: "Core / Titles",
};

interface Args {
    contents: string;
    color: Color;
    size: Size;
    bold: boolean;
    italic: boolean;
    underlined: boolean;
    p: boolean;
    link: boolean;
}

const DEFAULT_ARGS: Args = {
    contents: "hello",
    color: "red",
    size: "medium",
    bold: false,
    italic: false,
    underlined: false,
    p: false,
    link: false,
};

export const Ji = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;
    const { contents, ...titleProps } = props;

    return `<title-ji ${argsToAttrs(titleProps)}>${contents}</title-ji>`;
};

Ji.args = DEFAULT_ARGS;

Ji.argTypes = {
    color: {
        control: {
            type: "inline-radio",
            options: ["red", "blue", "green"],
        },
    },
    size: {
        control: {
            type: "inline-radio",
            options: [
                "small",
                "medium",
                "large",
                "subSmall",
                "subMedium",
                "subLarge",
            ],
        },
    },
};
