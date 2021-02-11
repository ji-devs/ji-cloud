import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/register/pages/step2";
import "@elements/entry/user/register/footer/login";
import "@elements/core/buttons/rectangle";
import "@elements/core/buttons/text";

const STR_SUBMIT = "One more step"
const STR_TERMS_LABEL = "I have read the terms and conditions (legal text…)";
const STR_LANGUAGE_LABEL = "Preferred language of communication*";
const STR_MARKETING_LABEL = "I would like to receive educational resources (GDPR legal text….)";

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
        <page-register-step2>
            <input-location slot="location"></input-location>
            <input-text slot="language" label="${STR_LANGUAGE_LABEL}" mode="text"></input-text>
            <input-checkbox slot="checkbox" label="${STR_TERMS_LABEL}"></input-checkbox>
            <input-checkbox slot="checkbox" label="${STR_MARKETING_LABEL}"></input-checkbox>
            <button-rect slot="submit" color="red" size="medium">${STR_SUBMIT}</button-rect> 

            <footer-register-login slot="footer"></footer-register-login>
        </page-register-step2>

    `
}

Step2.args = DEFAULT_ARGS;