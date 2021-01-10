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
    
    subtitle: string,
    subtitle_two: string,
    label: string,
  color: string,
  size: string,
  bold: boolean,
  italic: boolean
  

  
  }

  const DEFAULT_ARGS:LoginArgs = {
    
    subtitle: "You can now create, play, and share your content.",
    subtitle_two:"We are here to help you in whatever you need.",
    label: "Go to JI home",
    color: "red",
    size: "medium",
    bold: false,
    italic: false,
  }

  const STR_TITLE = "Welcome to JI Family"

export const Confirmation = (props?:LoginArgs) => {

    const {subtitle, subtitle_two, label, color, size, bold, italic} = props || DEFAULT_ARGS;


    return `
    <confirmation-full title="${STR_TITLE}">
        <sub-title slot="subtitle" title="${subtitle}" slot="subtitle"></sub-title>
        <sub-title slot="subtitle" title="${subtitle_two}" slot="subtitle"></sub-title>
        <div slot="button">${RectangleButton({label:label, color: color,size: size})}</div>       
    </confirmation-full>
    
    `
}

Confirmation.args = DEFAULT_ARGS;