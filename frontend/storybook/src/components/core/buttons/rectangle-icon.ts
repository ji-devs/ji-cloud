import {argsToAttrs} from "@utils/attributes";
import "@elements/core/buttons/rectangle-icon";
import { Color, Size } from "@elements/core/buttons/rectangle";
import {IconBefore, IconAfter } from "@elements/core/buttons/rectangle-icon";

export default {
  title: 'Core / Buttons',
}
interface Args {
    size: Size,
    color: Color,
    bold: boolean,
    italic: boolean,
    submit: boolean,
    disabled: boolean,

    iconBefore?: IconBefore,
    iconAfter?: IconAfter,

    contents: string,
}

const DEFAULT_ARGS:Args = {
    size: "medium",
    color: "red",
    bold: false,
    italic: false,
    submit: false,
    disabled: false,

    iconBefore: undefined,
    iconAfter: undefined,

    contents: "Submit",
}

export const RectangleIcon = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<button-rect-icon ${argsToAttrs(props)}>${props.contents}</button-rect-icon>`;
}

RectangleIcon.args = DEFAULT_ARGS;

RectangleIcon.argTypes = {
    size: {
        control: {
            type: 'inline-radio',
            options: ["small", "medium", "large"]
        }
    },
    color: {
        control: {
            type: 'inline-radio',
            options: ["red", "blue", "green"]
        }
    },
    iconBefore: {
        control: {
            type: 'inline-radio',
            options: [undefined, "magnifier", "share", "create", "play", "plus"]
        }
    },
    iconAfter: {
        control: {
            type: 'inline-radio',
            options: [undefined, "arrow", "done"]
        }
    },
}
