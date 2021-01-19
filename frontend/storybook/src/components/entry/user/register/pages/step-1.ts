import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/register/pages/step1";
import "@elements/entry/user/register/widgets/password-strength";
import "@elements/core/titles/ji";
import "@elements/core/dividers/or-divider";
import "@elements/core/buttons/rectangle";

const STR_TITLE = "Sign Up - Step 1";
const STR_FIRSTNAME = "First name";
const STR_PLCFIRSTNAME = "Type your first name";
const STR_LASTNAME = "Last name";
const STR_PLCLASTNAME = "Type your last name";
const STR_USERNAME = "Create a User Name*";
const STR_PLCUSER = "This will be your public name on JI";
const STR_18 = "I am over 18";
const STR_ACCOUNT = "Already have an account?";
const STR_CONTINUE = "Continue";
const STR_REGISTER = "Login";

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
        <page-register-step1 title="${STR_TITLE}">
          <input-text slot="topleft" label="${STR_FIRSTNAME}" placeholder="${STR_PLCFIRSTNAME}" ></input-text>
          <input-text slot="topright" label="${STR_LASTNAME}" placeholder="${STR_PLCLASTNAME}" ></input-text>
          <input-text slot="username" label="${STR_USERNAME}" placeholder="${STR_PLCUSER}"  ></input-text>
          <input-checkbox slot="checkbox" label="${STR_18}"></input-checkbox>
          <button-rect slot="submit" iconAfter="arrow" color="red" size="medium">
            ${STR_CONTINUE}
          </button-rect> 

          <title-ji slot="noaccount" color="black">${STR_ACCOUNT}</title-ji>
          <title-ji slot="noaccount" color="blue" link>${STR_REGISTER}</title-ji>
        </page-register-step1>

    `
}

Step1.args = DEFAULT_ARGS;