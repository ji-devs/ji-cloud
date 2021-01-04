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
    title: string,
    color: string,
    logintitle: string,
    forgottenPassword: string,
    passwordtitle:string,
    noaccount:string,
    helpertext:string,
    errormessage: string,
    instruction: boolean,
    error: boolean,
   

  
  }

  const DEFAULT_ARGS:LoginArgs = {
    title: "Sign up",
    logintitle: "Email",
    forgottenPassword: "",
    passwordtitle: "Create Password",
    noaccount: "Already have an account?",
    helpertext: "Your password looks good", 
    errormessage: "",
    instruction: false,
    error: false,
    color: "red",
  }

export const SignUpStart = (props?:LoginArgs) => {

    const {title,color, logintitle, forgottenPassword, passwordtitle, noaccount, helpertext,errormessage, instruction, error} = props || DEFAULT_ARGS;


    return `
    <login-full title="${title}">
        <div slot="google">${GoogleButton()}</div>
        <or-divider slot="divider"></or-divider>
        
        <input-text slot="username" label="${logintitle}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <password-strength slot="passwordstrength"></password-strength>
        <input-text slot="password" label="${passwordtitle}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <plain-blue title="${forgottenPassword}" slot="passwordreminder"></plain-blue>
        <div slot="submit">${RectangleButton()}</div>
        <plain-blue title="${noaccount}" slot="noaccount"></plain-blue>
    </login-full>
    
    `
}

SignUpStart.args = DEFAULT_ARGS;
