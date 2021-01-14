import "@elements/admin/templates-layout/login-full";
import "@elements/password-strength";

import "@elements/titles/underlined-title";
import "@elements/titles/plain-blue";
import "@elements/dividers/or-divider";
import {GoogleButton} from "~/components/special-buttons";
import { RectangleButton } from "~/components/rectangle-button";
import { Strength as Strengthbar} from "@elements/password-strength";
import { Mode } from "@elements/admin/templates-layout/login-full";

export default {
  title: 'Full Pages/Login',
}

interface LoginArgs {
  strength: Strengthbar,
  mode: Mode,
 
  }

  const DEFAULT_ARGS:LoginArgs = {
  strength: 0,
  mode: "none",
  }

  const STR_TITLE = "Sign Up";
  const STR_EMAIL = "Email";
  const STR_PLCEMAIL = "Type or paste your email";
  const STR_PASSWORD = "Create Password";
  const STR_PLCPASSWORD ="********";
  const STR_ACCOUNT = "Already have an account?";
  const STR_FORGOTTEN = "";
  const STR_REGISTER = "Login";
  const STR_HELPEMAIL = "Test";
  const STR_HELPPASSWORD = "8 Characters or more";

export const SignUpStart = (props?:LoginArgs) => {

    const {strength, mode} = props || DEFAULT_ARGS;


    return `
    <login-full title="${STR_TITLE}" mode="${mode}">
        <div slot="google">${GoogleButton()}</div>
        <or-divider slot="divider"></or-divider>
        
        <input-text slot="username" label="${STR_EMAIL}" mode="text" placeholder=${STR_PLCEMAIL}>
        </input-text>
        <password-strength slot="passwordstrength" strength ="${strength}"></password-strength>
        <input-text slot="password" label="${STR_PASSWORD}" mode="passwordHidden" placeholder="${STR_PLCPASSWORD}">
        </input-text>
        <plain-blue title="${STR_FORGOTTEN}" slot="passwordreminder"></plain-blue>
        <div slot="submit">${RectangleButton()}</div>
        <plain-black title="${STR_ACCOUNT}" slot="noaccount"></plain-black>
        <plain-blue title="${STR_REGISTER}" slot="noaccount"></plain-blue>
    </login-full>
    
    `
}

SignUpStart.args = DEFAULT_ARGS;
SignUpStart.argTypes = {
  strength: {
    control: {
      type: 'inline-radio',
      options: [1, 2, 3, 0]
    }
  },
  mode: {
    control: {
      type: 'inline-radio',
      options: ["weak", "average", "strong", "none"]
    }
  },
}
