import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/email/pages/email-confirm";
import "@elements/core/buttons/rectangle";

export default {
  title: 'Entry / User / Email / Pages',
}
const STR_TITLE = "Welcome to JI Family";
const STR_SUB ="You can now create, play, and share your content.";
const STR_SUBSUB = "We are here to help you in whatever you need.";
const STR_LABEL = "Go to JI home";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const EmailConfirm = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-email-confirm title="${STR_TITLE}">
            <title-ji size="subMedium" slot="subtitle">${STR_SUB}</title-ji>
            <title-ji size="subMedium" slot="subtitle">${STR_SUBSUB}</title-ji>
            <button-rect slot="button" color="red">${STR_LABEL}</button-rect>
        </page-email-confirm>
    `
}

EmailConfirm.args = DEFAULT_ARGS;