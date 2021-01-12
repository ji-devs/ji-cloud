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
 
  }

  const DEFAULT_ARGS:LoginArgs = {
 helpertext: "",
    errormessage: "",
    instruction: false,
    error:false,
  }

  const STR_NEWPASSWORD = "Create a New Password";
  const STR_LABEL = "Set Password";
  const STR_MEDIUM = "medium";
  const STR_RED = "red";
  const STR_ENTERPASSWORD = "Enter new Password";
  const STR_LOGGEDOUT = "You’ll be logged in automatically after this"

export const LoginForgotPassword = (props?:LoginArgs) => {

    const {helpertext,errormessage, instruction, error} = props || DEFAULT_ARGS;


    return `
    <login-full title="${STR_NEWPASSWORD}">
    
       
        <input-text slot="password" label="${STR_ENTERPASSWORD}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <div slot="submit">${RectangleButton({label:STR_LABEL, color: STR_RED,size: STR_MEDIUM, imgrighthidden:true, imglefthidden:true,})}</div>
        <plain-black title="${STR_LOGGEDOUT}" slot="noaccount"></plain-blue>
    </login-full>
    
    `
}

LoginForgotPassword.args = DEFAULT_ARGS;