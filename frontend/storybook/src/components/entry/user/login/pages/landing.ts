import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/login/pages/landing";
import "@elements/core/buttons/rectangle";
import "@elements/entry/user/_common/buttons/google";

export default {
  title: 'Entry / User / Login / Pages',
}

const STR_REGISTER = "Don't have an account yet?";
const STR_PASSWORD = "Password";
const STR_FORGOTTEN ="Forgot your Password?";
const STR_USERLABEL = "Email";
const STR_SUBMIT = "Submit";
const STR_PASSWORDPLACEHOLDER = "Type your password";
const STR_USERNAMEPLACEHOLDER = "Type your username";
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
            
            <input-wrapper slot="email" mode="text" label="${STR_USERLABEL}">
                <input placeholder="${STR_USERNAMEPLACEHOLDER}">
            </input-wrapper>
            <input-password slot="password" placeholder="${STR_PASSWORDPLACEHOLDER}" label="${STR_PASSWORD}" >
                <input>
            </input-password>
            <button-text color="blue" slot="password-forgot">${STR_FORGOTTEN}</button-text>
            <button-rect slot="submit" color="red" size="medium">${STR_SUBMIT}</button-rect> 
            <button-text color="blue" slot="register" weight="normal">${STR_REGISTER}</button-text>
        </page-login-landing>
    
    `
}

Landing.args = DEFAULT_ARGS;