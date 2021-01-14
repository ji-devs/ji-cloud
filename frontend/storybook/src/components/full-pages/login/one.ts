import "@elements/admin/templates-layout/login-full";
import "@elements/titles/underlined-title";
import "@elements/titles/plain-blue";
import "@elements/dividers/or-divider";
import {GoogleButton} from "~/components/special-buttons";
import { RectangleButton } from "~/components/rectangle-button";


export default {
  title: 'Full Pages/Login',
}

interface LoginArgs {
  
    instruction: boolean,
    error: string,
    
    errortwo:string,
   

  
  }

  const DEFAULT_ARGS:LoginArgs = {
    instruction: false,
    error: "",
   
    errortwo:""
  }

  const STR_ACCOUNT = "Don't have an account yet?";
  const STR_REGISTER = "Sign Up";
  const STR_TITLE = "Login";
  const STR_PASSWORD = "Password";
  const STR_FORGOTTEN ="Forgot your Password?";
  const STR_USERLABEL = "User Name";
  const STR_HELP ="Test";

export const LoginFullOne = (props?:LoginArgs) => {

    const {errortwo,error, instruction} = props || DEFAULT_ARGS;


    return `
    <login-full title="${STR_TITLE}">
        <div slot="google">${GoogleButton()}</div>
        <or-divider slot="divider"></or-divider>
        
        <input-text slot="username" mode="text" label="${STR_USERLABEL}" helpertext="${STR_HELP}" error="${error}" ${instruction && "instruction"}>
        </input-text>
        <input-text slot="password" mode="passwordHidden"  label="${STR_PASSWORD}" helpertext="${STR_HELP}" error="${errortwo}" ${instruction && "instruction"}>
        </input-text>
        <plain-blue title="${STR_FORGOTTEN}" slot="passwordreminder"></plain-blue>
        <div slot="submit">${RectangleButton()}</div>
        <plain-black title="${STR_ACCOUNT}" slot="noaccount"></plain-black>
        <plain-blue title="${STR_REGISTER}" slot="noaccount"></plain-blue>
    </login-full>
    
    `
}

LoginFullOne.args = DEFAULT_ARGS;


