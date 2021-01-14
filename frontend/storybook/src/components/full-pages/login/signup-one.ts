import "@elements/admin/templates-layout/signup-full";
import "@elements/password-strength";

import "@elements/titles/underlined-title";
import "@elements/titles/plain-blue";
import "@elements/dividers/or-divider";
import {GoogleButton} from "~/components/special-buttons";
import { RectangleButton } from "~/components/rectangle-button";
import { render } from "lit-html";


export default {
  title: 'Full Pages/Login',
}

interface LoginArgs {
   
    errorname: string,
    errorlast: string,
    erroruser:string,
    
    
  }

  const DEFAULT_ARGS:LoginArgs = {
    errorname: "",
    errorlast:"",
    erroruser:"",
   
    
    
  }

const STR_TITLE = "Sign Up - Step 1";
const STR_FIRSTNAME = "First name";
const STR_PLCFIRSTNAME = "Type your first name";
const STR_LASTNAME = "Last name";
const STR_PLCLASTNAME = "Type your last name";
const STR_USERNAME = "Create a User Name*";
const STR_PLCUSER = "This will be your public name on JI";
const STR_18 = "I am over 18";
const STR_ACCOUNT = "Already have an account?";
const STR_CONTINUE = "Continue";
const STR_REGISTER = "Login";
const STR_ARROW = "continue_arrow.svg";
const STR_MEDIUM = "medium";
const STR_RED = "red";


export const SignUpOne = (props?:LoginArgs) => {

    const { errorname,errorlast,erroruser} = props || DEFAULT_ARGS;


    return `
    <signup-full title="${STR_TITLE}">
        
        
        <input-text slot="topleft" label="${STR_FIRSTNAME}" placeholder="${STR_PLCFIRSTNAME}"  error="${errorname}">
        </input-text>
        <input-text slot="topright" label="${STR_LASTNAME}" placeholder="${STR_PLCLASTNAME}"  error="${errorlast}">
        </input-text>
        <input-text slot="username" label="${STR_USERNAME}" placeholder="${STR_PLCUSER}"  error="${erroruser}">
        </input-text>
        <input-checkbox slot="checkbox" label="${STR_18}">

  </input-checkbox>
        <div slot="submit">${RectangleButton({iconpath:STR_ARROW, color:STR_RED, size:STR_MEDIUM, label:STR_CONTINUE,bold:false,italic:false,imgrighthidden:false, imglefthidden:true})}</div>
        <plain-black title="${STR_ACCOUNT}" slot="noaccount"></plain-black>
        <plain-blue title="${STR_REGISTER}" slot="noaccount"></plain-blue>

        </signup-full>

    `
}

SignUpOne.args = DEFAULT_ARGS;