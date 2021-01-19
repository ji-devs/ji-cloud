import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/register/pages/step2";
import "@elements/core/titles/ji";
import "@elements/core/dividers/or-divider";
import "@elements/core/buttons/rectangle";

const STR_SUBMIT = "Submit"
const STR_TITLE = "Sign Up - Step 2";
const STR_ACCOUNT = "Already have an account?";
const STR_REGISTER = "Login";
const STR_COUNTRY = "Location*";
const STR_CITY = "City";
const STR_SCHOOL = "School/Organization*";
const STR_STATE = "State";
const STR_SUBTITLE = "Tell us more about yourself so that we can tailor";
const STR_SUBSUBTITE =  "the content according to your specific needs";
const STR_TERMS = "I have read the terms and conditions (legal text…)";
const STR_LANGUAGE = "Preferred language of communication*";
const STR_GDPR = "I would like to receive educational resources (GDPR legal text….)";

export default {
  title: 'Entry / User / Register / Pages',
}
interface Args {
}

const DEFAULT_ARGS:Args = {
}
  

export const Step2 = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-register-step2 title="${STR_TITLE}">
            <input-text slot="location" label="${STR_COUNTRY}" mode="text">
            </input-text>
            <spacer-fourty slot="location"></spacer-fourty>
            <input-text slot="username" label="${STR_LANGUAGE}" mode="text">
            </input-text>
            <input-checkbox slot="checkbox" label="${STR_TERMS}"></input-checkbox>
            <input-checkbox slot="checkbox" label="${STR_GDPR}"></input-checkbox>

            <button-rect slot="submit" color="red" size="medium">
                ${STR_SUBMIT}
            </button-rect> 

            <title-ji slot="noaccount" color="black">${STR_ACCOUNT}</title-ji>
            <title-ji slot="noaccount" color="blue" link>${STR_REGISTER}</title-ji>
        </page-register-step2>

    `
}

Step2.args = DEFAULT_ARGS;