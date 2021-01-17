import "@elements/admin/templates-layout/signup-full-wide";
import "@elements/password-strength";

import "@elements/titles/underlined-title";
import "@elements/titles/subtitle";
import "@elements/cards/grey-card";
import {Stream} from "~/components/lists/stream";
import "@elements/titles/plain-blue";
import { UserInfo } from "~/components/lists/user-info";

import { RectangleButton } from "~/components/rectangle-button";


export default {
  title: 'Full Pages/Login',
}

interface LoginArgs {
  }

  const DEFAULT_ARGS:LoginArgs = {
  }

  const STR_TITLE = "Sign Up - Step 3";
  const STR_SUBTITLE = "We want to tailor the content that you find to your interests and needs.";
  const STR_SUBSUBTITLE = "You can select as many as you like now and edit it later it in your profile page";

export const SignUpThree = (props?:LoginArgs) => {

    const {} = props || DEFAULT_ARGS;


    return `
    <signup-full-wide title="${STR_TITLE}">
        
        <sub-title slot="subtitle" title="${STR_SUBTITLE}"></sub-title>
        <sub-title slot="subtitle" title="${STR_SUBSUBTITLE}"></sub-title>
        <div slot="main">${UserInfo()}</div>
        <grey-card slot="main">${Stream()}</grey-card>

        <div slot="submit">${RectangleButton()}</div>

        </signup-full-wide>

    `
}

SignUpThree.args = DEFAULT_ARGS;