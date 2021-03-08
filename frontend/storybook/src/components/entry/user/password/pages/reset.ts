import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/password/pages/reset";
import "@elements/core/buttons/rectangle";
import "@elements/core/buttons/text";
import {Strength as PasswordStrength} from "@elements/entry/user/register/widgets/password-strength";

export default {
  title: 'Entry / User / Password / Pages',
}

const STR_SUBMIT = "Set Password";
const STR_PASSWORD_LABEL = "Enter new password";
const STR_PASSWORD_HELP ="8 Characters or longer"

interface Args {
  passwordStrength: PasswordStrength,
}

const DEFAULT_ARGS:Args = {
    passwordStrength: "none"
}

export const Reset = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {passwordStrength} = props;

    return `
        <page-password-reset passwordStrength="${passwordStrength}">
            <input-text slot="password" placeholder=${STR_PASSWORD_LABEL} label="${STR_PASSWORD_LABEL}" helpertext="${STR_PASSWORD_HELP}"  mode="passwordHidden"></input-text>
            <button-rect slot="submit" color="red" size="medium">${STR_SUBMIT}</button-rect> 
        </page-password-reset>
    `
}

Reset.args = DEFAULT_ARGS;

Reset.argTypes = {
    passwordStrength: {
        control: {
            type: 'inline-radio',
            options: ["none", "weak", "average", "strong"]
        }
    }
}