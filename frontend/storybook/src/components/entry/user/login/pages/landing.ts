import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/login/pages/landing";
import "@elements/core/titles/ji";
import "@elements/core/buttons/rectangle";
import "@elements/entry/user/_common/buttons/google";

export default {
  title: 'Entry / User / Login / Pages',
}

const STR_ACCOUNT = "Don't have an account yet?";
const STR_REGISTER = "Sign Up";
const STR_TITLE = "Login";
const STR_PASSWORD = "Password";
const STR_FORGOTTEN ="Forgot your Password?";
const STR_USERLABEL = "User Name";
const STR_SUBMIT = "Submit";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Landing = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {} = props;

    return `
        <page-login-landing>

            <button-google slot="google"></button-google>
            
            <input-text slot="username" mode="text" label="${STR_USERLABEL}" }></input-text>
            <input-text slot="password" mode="passwordHidden" label="${STR_PASSWORD}" ></input-text>
            <button-text color="blue" slot="password-forgot">${STR_FORGOTTEN}</button-text>
            <button-rect slot="submit" color="red" size="medium">${STR_SUBMIT}</button-rect> 
            <button-text color="blue" slot="register">${STR_REGISTER}</button-text>
        </page-login-landing>
    
    `
}

Landing.args = DEFAULT_ARGS;