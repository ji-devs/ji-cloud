import "@elements/admin/templates-layout/signup-full-wide";
import "@elements/password-strength";

import "@elements/titles/underlined-title";
import "@elements/titles/subtitle";
import "@elements/cards/grey-card";


import "@elements/titles/plain-blue";
import { UserInfo } from "~/components/full-pages/login/user-info";

import { RectangleButton } from "~/components/rectangle-button";


export default {
  title: 'Full Pages/Login',
}

interface LoginArgs {
    title:string,
    subtitle:string,
    subtitletwo:string,
  }

  const DEFAULT_ARGS:LoginArgs = {
    title: "Sign up - step 3",
    subtitle:"We want to tailor the content that you find to your interests and needs.",
    subtitletwo: "You can select as many as you like now and edit it later it in your profile page",

   
  }

export const SignUpThree = (props?:LoginArgs) => {

    const {title,subtitle,subtitletwo} = props || DEFAULT_ARGS;


    return `
    <signup-full-wide title="${title}">
        
        <sub-title slot="subtitle" title="${subtitle}"></sub-title>
        <sub-title slot="subtitle" title="${subtitletwo}"></sub-title>
        <div slot="main">${UserInfo}</div>

        <div slot="submit">${RectangleButton()}</div>

        </signup-full-wide>

    `
}

SignUpThree.args = DEFAULT_ARGS;