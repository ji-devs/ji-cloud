import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/register/pages/start";
import "@elements/entry/user/register/widgets/password-strength";
import "@elements/core/titles/ji";
import "@elements/core/dividers/or-divider";
import "@elements/core/buttons/rectangle";
import "@elements/entry/user/_common/buttons/google";

import {Strength as PasswordStrength} from "@elements/entry/user/register/widgets/password-strength";

export default {
  title: 'Entry / User / Register / Pages',
}

const STR_TITLE = "Sign Up";
const STR_SUBMIT = "Submit";
const STR_EMAIL = "Email";
const STR_PLCEMAIL = "Type or paste your email";
const STR_PASSWORD = "Create Password";
const STR_PLCPASSWORD ="********";
const STR_ACCOUNT = "Already have an account?";
const STR_REGISTER = "Login";
const STR_HELPEMAIL = "Test";
const STR_HELPPASSWORD = "8 Characters or more";

interface Args {
  passwordStrength: PasswordStrength,
}

const DEFAULT_ARGS:Args = {
    passwordStrength: "none"
}

export const Start = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {passwordStrength} = props;

    return `
        <page-register-start title="${STR_TITLE}" passwordStrength=${passwordStrength}>
            <button-google slot="google"></button-google>
            <or-divider slot="divider"></or-divider>
            
            <input-text slot="username" label="${STR_EMAIL}" mode="text" placeholder=${STR_PLCEMAIL}>
            </input-text>
            <input-text slot="password" label="${STR_PASSWORD}" mode="passwordHidden" placeholder="${STR_PLCPASSWORD}">
            </input-text>
            <button-rect slot="submit" color="red" size="medium">
              ${STR_SUBMIT}
            </button-rect> 

            <title-ji slot="noaccount" color="black">${STR_ACCOUNT}</title-ji>
            <title-ji slot="noaccount" color="blue" link>${STR_REGISTER}</title-ji>
        </page-register-start>
    
    `
}

Start.args = DEFAULT_ARGS;

Start.argTypes = {
    passwordStrength: {
        control: {
            type: 'inline-radio',
            options: ["none", "weak", "average", "strong"]
        }
    }
}