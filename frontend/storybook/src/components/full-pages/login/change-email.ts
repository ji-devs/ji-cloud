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
   
    color: string,
    passwordtitle:string,
    helpertext:string,
    errormessage: string,
    instruction: boolean,
    error: boolean,
    label: string,
    size:string

  
  }

  const DEFAULT_ARGS:LoginArgs = {
    
    label: "Email me to the new address",
    passwordtitle: "Email",
    color: "red",
    helpertext: "",
    errormessage: "",
    instruction: false,
    size:"medium"
  }

  const STR_TITLE ="Change Email Account";
  const STR_SUB = "This is the email that you filled in. You can change it now."

export const LoginChangeEmail = (props?:LoginArgs) => {

    const {color, size, passwordtitle,label, helpertext,errormessage, instruction, error} = props || DEFAULT_ARGS;


    return `
    <login-full title="${STR_TITLE}">
    
    <plain-black title="${STR_SUB}" slot="sub"></plain-black>
        <input-text slot="username" label="${passwordtitle}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        
        <div slot="submit">${RectangleButton({label:label, color: color,size: size, imgrighthidden:true, imglefthidden:true})}</div>
    </login-full>
    
    `
}

LoginChangeEmail.args = DEFAULT_ARGS;