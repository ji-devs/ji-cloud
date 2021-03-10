import {argsToAttrs} from "@utils/attributes";
import "@elements/core/buttons/sidebar";
import {MODE} from "@elements/core/buttons/sidebar";
export default {
  title: 'Core / Buttons',
}

interface Args {
    mode: MODE
}

const DEFAULT_ARGS:Args = {
    mode: "keyboard"
}

export const Sidebar = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<button-sidebar ${argsToAttrs(props)}></button-sidebar>`
}

Sidebar.args = DEFAULT_ARGS;

Sidebar.argTypes = {
    mode: {
        control: {
            type: 'inline-radio',
            options: ["keyboard", "dicta", "sefaria"]
        }
    }
}

