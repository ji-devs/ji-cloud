import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/email/pages/email-change";
import "@elements/core/buttons/rectangle";

export default {
  title: 'Entry / User / Email / Pages',
}

const STR_EMAIL_LABEL = "Email";
const STR_EMAIL_HELP ="Test";
const STR_BUTTON= "Email me to the new address";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const EmailChange = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-email-change>
            <input-text slot="email" label="${STR_EMAIL_LABEL}" helpertext="${STR_EMAIL_HELP}" mode="text"></input-text>
            <button-rect slot="submit" color="red">${STR_BUTTON}</button-rect>
        </page-email-change>
    `
}

EmailChange.args = DEFAULT_ARGS;