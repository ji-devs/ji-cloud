import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/password/pages/reset";
import "@elements/core/titles/ji";
import "@elements/core/dividers/or-divider";
import "@elements/core/buttons/rectangle";
import "@elements/entry/user/_common/buttons/google";

export default {
  title: 'Entry / User / Password / Pages',
}

const STR_NEWPASSWORD = "Create a New Password";
const STR_LABEL = "Set Password";
const STR_ENTERPASSWORD = "Enter new Password";
const STR_LOGGEDOUT = "You’ll be logged in automatically after this";
const STR_HELP ="8 Characters or longer"

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Reset = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {} = props;

    return `
        <page-password-reset title="${STR_NEWPASSWORD}">
            <input-text slot="password" label="${STR_ENTERPASSWORD}" helpertext="${STR_HELP}"  mode="passwordHidden">
            </input-text>

            <button-rect slot="submit" color="red" size="medium">
              ${STR_LABEL}
            </button-rect> 
            <title-ji color="black" slot="noaccount">${STR_LOGGEDOUT}</title-ji>
        </page-password-reset>
    
    `
}

Reset.args = DEFAULT_ARGS;