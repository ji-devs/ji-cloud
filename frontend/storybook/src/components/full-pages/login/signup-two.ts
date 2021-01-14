import "@elements/admin/templates-layout/signup-full";
import "@elements/password-strength";

import "@elements/titles/underlined-title";
import "@elements/titles/subtitle";

import "@elements/titles/plain-blue";
import "@elements/dividers/or-divider";
import {GoogleButton} from "~/components/special-buttons";
import { RectangleButton } from "~/components/rectangle-button";


export default {
  title: 'Full Pages/Login',
}

interface LoginArgs {
    errormessage: string,
    error: boolean,
    errorwrapper:boolean
   
  }

  const DEFAULT_ARGS:LoginArgs = {
    errormessage: "",
    error: false,
    errorwrapper:false

      
  }

  const STR_TITLE = "Sign Up - Step 2";
  const STR_ACCOUNT = "Already have an account?";
  const STR_REGISTER = "Login";
  const STR_COUNTRY = "Country";
  const STR_CITY = "City";
  const STR_SCHOOL = "School/Organization*";
  const STR_STATE = "State";
  const STR_SUBTITLE = "Tell us more about yourself so that we can tailor";
  const STR_SUBSUBTITE =  "the content according to your specific needs";
  const STR_TERMS = "I have read the terms and conditions (legal text…)";
  const STR_LANGUAGE = "Preferred language of communication*";
  const STR_GDPR = "I would like to receive educational resources (GDPR legal text….)";
  

export const SignUpTwo = (props?:LoginArgs) => {

    const {errormessage, error,errorwrapper } = props || DEFAULT_ARGS;


    return `
    <signup-full title="${STR_TITLE}">
        
        <sub-title slot="subtitle" title="${STR_SUBTITLE}"></sub-title>
        <sub-title slot="subtitle" title="${STR_SUBSUBTITE}"></sub-title>
        <dropdown-select slot="topleft" label="${STR_COUNTRY}"  error="${errormessage}" ${error && "error"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <dropdown-select slot="topright"  label="${STR_STATE}" error="${errormessage}"  ${error && "error"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <dropdown-select slot="bottomleft" label="${STR_CITY}" error="${errormessage}"  ${error && "error"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <dropdown-select slot="bottomright" label="${STR_SCHOOL}" error="${errormessage}" ${error && "error"} ${errorwrapper && "errorwrapper"} >
        </dropdown-select>
        
        <input-text slot="username" label="${STR_LANGUAGE}" mode="text" error="${errormessage}">
        </input-text>
        <input-checkbox slot="checkbox" label="${STR_TERMS}"></input-checkbox>
        <input-checkbox slot="checkbox" label="${STR_GDPR}"></input-checkbox>

        <div slot="submit">${RectangleButton()}</div>
        <plain-black title="${STR_ACCOUNT}" slot="noaccount"></plain-black>
        <plain-blue title="${STR_REGISTER}" slot="noaccount"></plain-blue>

        </signup-full>

    `
}

SignUpTwo.args = DEFAULT_ARGS;