import "@elements/admin/templates-layout/confirmation-full";
import "@elements/titles/underlined-title";
import "@elements/titles/plain-black";
import "@elements/titles/subtitle";
import "@elements/buttons/confirmation";

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

  const STR_TITLE = "Welcome to JI Family";
  const STR_SUB ="You can now create, play, and share your content.";
  const STR_SUBSUB = "We are here to help you in whatever you need.";
  const STR_LABEL = "Go to JI home";
  const STR_MEDIUM = "medium";
  const STR_RED = "red";

export const Confirmation = (props?:LoginArgs) => {

    const {} = props || DEFAULT_ARGS;


    return `
    <confirmation-full title="${STR_TITLE}">
        <sub-title slot="subtitle" title="${STR_SUB}" slot="subtitle"></sub-title>
        <sub-title slot="subtitle" title="${STR_SUBSUB}" slot="subtitle"></sub-title>
        <div slot="button">${RectangleButton({label:STR_LABEL, color: STR_RED,size: STR_MEDIUM,imglefthidden:true, imgrighthidden:true,bold:false, italic:false, iconpath:""})}</div>       
    </confirmation-full>
    
    `
}

Confirmation.args = DEFAULT_ARGS;