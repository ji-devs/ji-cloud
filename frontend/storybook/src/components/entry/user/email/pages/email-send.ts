import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/email/pages/email-send";
import "@elements/core/titles/ji";
import "@elements/core/dividers/or-divider";
import "@elements/core/buttons/rectangle";
import "@elements/core/lists/list-vertical";
import { Mode as ConfirmationMode }  from "@elements/entry/user/email/buttons/email-send";
import "@elements/entry/user/email/buttons/email-send";

export default {
  title: 'Entry / User / Email / Pages',
}
const STR_TITLE = "We Just Sent You an Email";
const STR_SUBTITLE = "Open the email and click on the Verification button";
const STR_SUBSUBTITLE = "It may have been filtered into the promotion or spam folders";
const STR_CHANGE = "Change email account";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const EmailSend = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-email-send title="${STR_TITLE}">
            <title-ji slot="subtitle" size="subMedium">${STR_SUBTITLE}</title-ji>
            <title-ji slot="subtitle" size="subMedium">${STR_SUBSUBTITLE}</title-ji>
            <spacer-fourty slot="confirmation"></spacer-fourty>
            <title-ji slot="main" color="blue" link>${STR_CHANGE}</title-ji>
            <contact-email slot="contact"></contact-email>
        </page-email-send>
    `
}

EmailSend.args = DEFAULT_ARGS;