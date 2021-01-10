import "@elements/admin/templates-layout/login-full";
import "@elements/titles/underlined-title";
import "@elements/titles/plain-black";

import { RectangleButton } from "~/components/rectangle-button";
import { colorStyles } from "@elements/_styles/colors";
import { render } from "lit-html";


export default {
  title: 'Full Pages/Login',
}

interface LoginArgs {
    helpertext:string,
    errormessage: string,
    instruction: boolean,
    error: boolean,
    errorwrapper: boolean;
  }

  const DEFAULT_ARGS:LoginArgs = {
    helpertext: "",
    errormessage: "",
    instruction: false,
    error:false,
    errorwrapper:false,
  }

  const STR_TITLE ="Change Email Account";
  const STR_SUB = "This is the email that you filled in. You can change it now.";
  const STR_PASSWORDLABEL = "Email";
  const STR_BTNLABEL = "Email me to the new address";
  const STR_RED = "red";
  const STR_MEDIUM = "medium";
  const STR_FALSE = "false";

export const LoginChangeEmail = (props?:LoginArgs) => {

    const {helpertext,errormessage, instruction, error, errorwrapper} = props || DEFAULT_ARGS;


    return `
    <login-full title="${STR_TITLE}">
    
    <plain-black title="${STR_SUB}" slot="sub"></plain-black>
        <input-text slot="username" label="${STR_PASSWORDLABEL}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${errorwrapper && "errorwrapper"} ${error && "error"} imghidden="${STR_FALSE}">
        </input-text>
        
        <div slot="submit">${RectangleButton({label:STR_BTNLABEL, color: STR_RED,size: STR_MEDIUM, imgrighthidden:true, imglefthidden:true})}</div>
    </login-full>
    
    `
}

LoginChangeEmail.args = DEFAULT_ARGS;