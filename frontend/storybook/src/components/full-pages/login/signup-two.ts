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
    helpertext:string,
    errormessage: string,
    instruction: boolean,
    error: boolean,
    imghidden:boolean,
  }

  const DEFAULT_ARGS:LoginArgs = {
      helpertext: "Your password looks good", 
    errormessage: "",
    instruction: false,
    error: false,
    imghidden:true
      
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

    const {helpertext,errormessage, instruction, error, imghidden} = props || DEFAULT_ARGS;


    return `
    <signup-full title="${STR_TITLE}">
        
        <sub-title slot="subtitle" title="${STR_SUBTITLE}"></sub-title>
        <sub-title slot="subtitle" title="${STR_SUBSUBTITE}"></sub-title>
        <dropdown-select slot="topleft" label="${STR_COUNTRY}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <dropdown-select slot="topright"  label="${STR_STATE}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <dropdown-select slot="bottomleft" label="${STR_CITY}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <dropdown-select slot="bottomright" label="${STR_SCHOOL}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        
        <input-text slot="username" label="${STR_LANGUAGE}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} ${imghidden && "imghidden"} >
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