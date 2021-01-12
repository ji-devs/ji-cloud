import "@elements/admin/templates-layout/login-full";
import "@elements/password-strength";

import "@elements/titles/underlined-title";
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
    visiblepassword:boolean,
   

  
  }

  const DEFAULT_ARGS:LoginArgs = {
    
    helpertext: "Your password looks good", 
    errormessage: "",
    instruction: false,
    error: false,
    imghidden:true,
    visiblepassword:true,
  }

  const STR_TITLE = "Sign Up";
  const STR_EMAIL = "Email";
  const STR_PLCEMAIL = "Type or paste your email";
  const STR_PASSWORD = "Create Password";
  const STR_PLCPASSWORD ="********";
  const STR_ACCOUNT = "Already have an account?";
  const STR_FORGOTTEN = "";
  const STR_ICON = "icn-show-idle.svg"

export const SignUpStart = (props?:LoginArgs) => {

    const {helpertext,errormessage, instruction, error, visiblepassword} = props || DEFAULT_ARGS;


    return `
    <login-full title="${STR_TITLE}">
        <div slot="google">${GoogleButton()}</div>
        <or-divider slot="divider"></or-divider>
        
        <input-text slot="username" label="${STR_EMAIL}" placeholder=${STR_PLCEMAIL} helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <password-strength slot="passwordstrength"></password-strength>
        <input-text slot="password" label="${STR_PASSWORD}" mode="password" ${visiblepassword && "visiblepassword"} placeholder="${STR_PLCPASSWORD}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"}>
        </input-text>
        <plain-blue title="${STR_FORGOTTEN}" slot="passwordreminder"></plain-blue>
        <div slot="submit">${RectangleButton()}</div>
        <plain-blue title="${STR_ACCOUNT}" slot="noaccount"></plain-blue>
    </login-full>
    
    `
}

SignUpStart.args = DEFAULT_ARGS;
