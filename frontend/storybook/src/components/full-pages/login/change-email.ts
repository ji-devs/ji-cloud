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
    helpertext:string,
    errormessage: string,
    instruction: boolean,
    error: boolean,
    label: string,
    size:string

  
  }

  const DEFAULT_ARGS:LoginArgs = {
    title: "Change Email account",
    label: "Email me to the new address",
    passwordtitle: "Email",
    color: "red",
    helpertext: "",
    errormessage: "",
    instruction: false,
    size:"medium"
  }

export const LoginChangeEmail = (props?:LoginArgs) => {

    const {title,color, size, passwordtitle,label, helpertext,errormessage, instruction, error} = props || DEFAULT_ARGS;


    return `
    <login-full title="${title}">
    
       
        <input-text slot="username" label="${passwordtitle}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <div slot="submit">${RectangleButton({label:label, color: color,size: size})}</div>
    </login-full>
    
    `
}

LoginChangeEmail.args = DEFAULT_ARGS;