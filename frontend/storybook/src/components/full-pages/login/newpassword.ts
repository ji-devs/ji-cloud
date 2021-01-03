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
    title: string,
    color: string,
    passwordtitle:string,
    noaccount:string,
    helpertext:string,
    errormessage: string,
    instruction: boolean,
    error: boolean,
    label: string,
    size:string

  
  }

  const DEFAULT_ARGS:LoginArgs = {
    title: "Create a new password",
    label: "Set Password",
    passwordtitle: "Enter new Password",
    noaccount: "You’ll be logged in automatically after this",
    color: "red",
    helpertext: "",
    errormessage: "",
    instruction: false,
    size:"medium"
  }

export const LoginForgotPassword = (props?:LoginArgs) => {

    const {title,color, size, passwordtitle,label, noaccount, helpertext,errormessage, instruction, error} = props || DEFAULT_ARGS;


    return `
    <login-full title="${title}">
    
       
        <input-text slot="password" label="${passwordtitle}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <div slot="submit">${RectangleButton({label:label, color: color,size: size})}</div>
        <plain-black title="${noaccount}" slot="noaccount"></plain-blue>
    </login-full>
    
    `
}

LoginForgotPassword.args = DEFAULT_ARGS;