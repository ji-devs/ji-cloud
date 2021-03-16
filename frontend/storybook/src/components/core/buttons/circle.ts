import {argsToAttrs} from "@utils/attributes";
import "@elements/core/buttons/circle";

export default {
  title: 'Core / Buttons',
}
interface Args {
    color?: 'blue' | 'green';
    label: string,
    contents: string
}

const DEFAULT_ARGS:Args = {
    color: "blue",
    label: "label here",
    contents: "1"
}

export const Circle = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {contents, ...buttonProps} = props;
    return `<button-circle ${argsToAttrs(buttonProps)}>${contents}</button-circle>`;
}

Circle.args = DEFAULT_ARGS;

Circle.argTypes = {
    color: {
        control: {
            type: 'inline-radio',
            options: [undefined, "blue", "green"]
        }
    }
}
