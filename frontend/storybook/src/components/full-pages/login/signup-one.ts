import "@elements/admin/templates-layout/signup-full";
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
    firstname: string,
    username: string,
    lastname:string,
    noaccount:string,
    helpertext:string,
    errormessage: string,
    instruction: boolean,
    error: boolean,
    checkbox_label:string,
   

  
  }

  const DEFAULT_ARGS:LoginArgs = {
    title: "Sign up - step 1",
    firstname: "First name",
    username: "Create a username",
    lastname: "Family name",
    noaccount: "Already have an account?",
    helpertext: "Your password looks good", 
    errormessage: "",
    instruction: false,
    error: false,
    color: "red",
    checkbox_label:"I am over 18"
  }

export const SignUpOne = (props?:LoginArgs) => {

    const {title,color,checkbox_label, firstname, username, lastname, noaccount, helpertext,errormessage, instruction, error} = props || DEFAULT_ARGS;


    return `
    <signup-full title="${title}">
        
        
        <input-text slot="topleft" label="${firstname}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <input-text slot="topright" label="${lastname}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <input-text slot="username" label="${username}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <input-checkbox slot="checkbox" label="${checkbox_label}">

  </input-checkbox>
        <div slot="submit">${RectangleButton()}</div>
        <plain-blue title="${noaccount}" slot="noaccount"></plain-blue>

        </signup-full>

    `
}

SignUpOne.args = DEFAULT_ARGS;