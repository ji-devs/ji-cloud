import {argsToAttrs, deleteNone} from "@utils/attributes";
import "@elements/core/modals/confirm";
import {ConfirmMode} from "@elements/core/modals/confirm";

export default {
  title: 'Core / Modals',
}

interface Args {
    mode: ConfirmMode
}

const DEFAULT_ARGS:Args = {
    mode: "deleteModule"
}

export const Confirm = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<modal-confirm ${argsToAttrs(props)}></model-confirm>`
}

//Continuing the previous example
Confirm.argTypes = {
  mode: {
    control: {
      type: 'inline-radio',
      options: ["deleteModule"]
    }
  },
}

Confirm.args = DEFAULT_ARGS;
