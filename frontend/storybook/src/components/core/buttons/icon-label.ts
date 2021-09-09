import {argsToAttrs} from "@utils/attributes";
import "@elements/core/buttons/icon-label";
import {IconKind} from "@elements/core/buttons/icon";

export default {
  title: 'Core / Buttons',
}

interface Args {
    icon: IconKind,
    label: string 
}

const DEFAULT_ARGS:Args = {
    icon: "gears",
    label: "Add something"
}

export const IconLabel = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<button-icon-label ${argsToAttrs(props)}></button-icon-label>`
}

IconLabel.args = DEFAULT_ARGS;

IconLabel.argTypes = {
    icon: {
        control: {
            type: 'inline-radio',
            options: ["circle-x-blue", "circle-+-blue", "audio", "audio-stop", "circle-check", "circle-kebab-grey", "circle-kebab-blue", "circle-pencil", "gears", "x"]
        }
    }
}
