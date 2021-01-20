import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/email/pages/email-confirm";
import "@elements/core/buttons/rectangle";

export default {
  title: 'Entry / User / Email / Pages',
}

const STR_BUTTON = "Go to JI home";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const EmailConfirm = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-email-confirm>
            <button-rect slot="button" color="red">${STR_BUTTON}</button-rect>
        </page-email-confirm>
    `
}

EmailConfirm.args = DEFAULT_ARGS;