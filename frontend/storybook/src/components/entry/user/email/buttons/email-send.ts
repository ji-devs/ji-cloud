import {argsToAttrs} from "@utils/attributes";
import { Mode}  from "@elements/entry/user/email/buttons/email-send";
import "@elements/entry/user/email/buttons/email-send";

export default {
  title: 'Entry / User / Email / Buttons',
}

interface Args {
    mode:Mode,
}

const DEFAULT_ARGS:Args = {
    mode: "sent"
}

export const EmailSend = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<button-email-send ${argsToAttrs(props)}></button-email-send>`;
}

EmailSend.args = DEFAULT_ARGS;

EmailSend.argTypes = {
  mode: {
    control: {
      type: 'inline-radio',
      options: ["send", "sent"]
    }
  }
}