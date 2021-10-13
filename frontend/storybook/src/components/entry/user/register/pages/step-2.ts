import {argsToAttrs} from "@utils/attributes";

import "@elements/entry/user/register/pages/step2";
import "@elements/entry/user/register/footer/login";
import "@elements/core/buttons/rectangle";
import "@elements/core/inputs/wrapper";

const STR_SUBMIT = "One more step"
const STR_TERMS_LABEL = "I have read the terms and conditions (legal text…)";
const STR_MARKETING_LABEL = "I would like to receive educational resources (GDPR legal text….)";

const STR_PROTECTING_PRIVACY = "Jewish Interactive (Ji) is committed to protecting and respecting your privacy, and we’ll only use your personal information to administer your account and to provide the products and services you requested from us.";
const STR_LOCATION_LABEL = "Location*";
const STR_PERSONA_LABEL = "I sign up as a...*";
const STR_ORGANIZATION_LABEL = "School/Organization*";
const STR_LANGUAGE_LABEL = "Language of communication*";


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
            <input-wrapper slot="location" label="${STR_LOCATION_LABEL}"">
                <input-location></input-location>
            </input-wrapper>

            <input-select slot="language" label="${STR_LANGUAGE_LABEL}" value="English">
                <input-select-option>Spanish</input-select-option>
            </input-select>
            <input-select slot="persona" label="${STR_PERSONA_LABEL}" value="Teacher" >
                <input-select-option>Student</input-select-option>
            </input-select>
            <input-wrapper slot="organization" label="${STR_ORGANIZATION_LABEL}"">
                <input type="text" />
            </input-wrapper>
            <input-checkbox slot="checkbox" label="${STR_TERMS_LABEL}"></input-checkbox>
            <input-checkbox slot="checkbox" label="${STR_MARKETING_LABEL}"></input-checkbox>
            <p slot="committed-to-privacy">${STR_PROTECTING_PRIVACY}</p>
            <button-rect slot="submit" color="red" size="medium">${STR_SUBMIT}</button-rect> 

            <footer-register-login slot="footer"></footer-register-login>
        </page-register-step2>

    `
}

Step2.args = DEFAULT_ARGS;