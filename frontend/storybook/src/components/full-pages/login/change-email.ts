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

  }

  const DEFAULT_ARGS:LoginArgs = {
   
  }

  const STR_TITLE ="Change Email Account";
  const STR_SUB = "This is the email that you filled in. You can change it now.";
  const STR_PASSWORDLABEL = "Email";
  const STR_BTNLABEL = "Email me to the new address";
  const STR_RED = "red";
  const STR_MEDIUM = "medium";
  const STR_HELP ="Test";
  
  
 

export const LoginChangeEmail = (props?:LoginArgs) => {

    const {} = props || DEFAULT_ARGS;


    return `
    <login-full title="${STR_TITLE}">
    
    <plain-black title="${STR_SUB}" slot="sub"></plain-black>
        <input-text slot="username" label="${STR_PASSWORDLABEL}" helpertext="${STR_HELP}" mode="text">
        </input-text>
        
        <div slot="submit">${RectangleButton({label:STR_BTNLABEL, color: STR_RED,size: STR_MEDIUM, imgrighthidden:true, imglefthidden:true, bold:false,italic:false,iconpath:""})}</div>
        <contact-email slot="contact"></contact-email>
        </login-full>
    
    `
}

LoginChangeEmail.args = DEFAULT_ARGS;
