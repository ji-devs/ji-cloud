import "@elements/admin/templates-layout/login-full";
import "@elements/titles/underlined-title";
import "@elements/titles/plain-black";

import { RectangleButton } from "~/components/rectangle-button";
import { colorStyles } from "@elements/_styles/colors";
import { render } from "lit-html";
import { TreeDropdown } from "~/components/dropdown";


export default {
  title: 'Full Pages/Login',
}

interface LoginArgs {
    instruction: boolean,
    error: string,
   
   

  }

  const DEFAULT_ARGS:LoginArgs = {
    instruction: false,
    error:"",
    
    
    
  }

  const STR_NEWPASSWORD = "Create a New Password";
  const STR_LABEL = "Set Password";
  const STR_MEDIUM = "medium";
  const STR_RED = "red";
  const STR_ENTERPASSWORD = "Enter new Password";
  const STR_LOGGEDOUT = "You’ll be logged in automatically after this";
  const STR_HELP ="8 Characters or longer"
  

export const LoginForgotPassword = (props?:LoginArgs) => {

    const {instruction, error} = props || DEFAULT_ARGS;


    return `
    <login-full title="${STR_NEWPASSWORD}">
    
       
        <input-text slot="password" label="${STR_ENTERPASSWORD}" helpertext="${STR_HELP}" error="${error}" ${instruction && "instruction"} ${error && "error"}  mode="passwordHidden">
        </input-text>
        <div slot="submit">${RectangleButton({label:STR_LABEL, color: STR_RED,size: STR_MEDIUM, imgrighthidden:true, imglefthidden:true,bold:false, italic:false, path:""})}</div>
        <plain-black title="${STR_LOGGEDOUT}" slot="noaccount"></plain-blue>
    </login-full>
    
    `
}

LoginForgotPassword.args = DEFAULT_ARGS;