import {argsToAttrs} from "@utils/attributes";
import "@elements/core/buttons/expand";
import {Mode} from "@elements/core/buttons/expand";

export default {
  title: 'Core / Buttons',
}

interface Args {
    mode: Mode
}

const DEFAULT_ARGS:Args = {
    mode: "expanded",
}

export const Expand = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<button-expand ${argsToAttrs(props)}></button-expand>`
}

Expand.args = DEFAULT_ARGS;

Expand.argTypes = {
    mode: {
        control: {
            type: 'inline-radio',
            options: ["expanded", "collapsed"]
        }
    }
}