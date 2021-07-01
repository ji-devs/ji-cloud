import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/register/pages/step1";
import "@elements/entry/user/register/footer/login";
import "@elements/core/buttons/rectangle";

const STR_FIRSTNAME_LABEL = "First name";
const STR_FIRSTNAME_PLACEHOLDER = "Type your first name";
const STR_LASTNAME_LABEL = "Last name";
const STR_LASTNAME_PLACEHOLDER = "Type your last name";
const STR_USERNAME_LABEL = "Create a User Name*";
const STR_USERNAME_PLACEHOLDER = "This will be your public name on JI";
const STR_18 = "I am over 18";
const STR_CONTINUE = "Continue";

export default {
  title: 'Entry / User / Register / Pages',
}
interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Step1 = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-register-step1>
          <input-wrapper slot="topleft" label="${STR_FIRSTNAME_LABEL}">
            <input placeholder="${STR_FIRSTNAME_PLACEHOLDER}">
          </input-wrapper>
          <input-wrapper slot="topright" label="${STR_LASTNAME_LABEL}">
            <input placeholder="${STR_LASTNAME_PLACEHOLDER}">
          </input-wrapper>
          <input-wrapper slot="username" label="${STR_USERNAME_LABEL}">
            <input placeholder="${STR_USERNAME_PLACEHOLDER}">
          </input-wrapper>
          <input-checkbox slot="checkbox" label="${STR_18}"></input-checkbox>
          <button-rect slot="submit" iconAfter="arrow" color="red" size="medium">${STR_CONTINUE}</button-rect> 
          <footer-register-login slot="footer"></footer-register-login>
        </page-register-step1>
    `
}

Step1.args = DEFAULT_ARGS;