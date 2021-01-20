import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/login/pages/landing";
import "@elements/core/buttons/rectangle";
import "@elements/entry/user/_common/buttons/google";
import "@elements/entry/user/login/footer/register";

export default {
  title: 'Entry / User / Login / Pages',
}

const STR_PASSWORD = "Password";
const STR_FORGOTTEN ="Forgot your Password?";
const STR_EMAIL = "Email";
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
            
            <input-text slot="email" mode="text" label="${STR_EMAIL}" }></input-text>
            <input-text slot="password" mode="passwordHidden" label="${STR_PASSWORD}" ></input-text>
            <button-text color="blue" slot="password-forgot">${STR_FORGOTTEN}</button-text>
            <button-rect slot="submit" color="red" size="medium">${STR_SUBMIT}</button-rect> 
            <footer-login-register slot="footer"></footer-login-register>
        </page-login-landing>
    
    `
}

Landing.args = DEFAULT_ARGS;