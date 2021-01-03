import "@elements/admin/templates-layout/signup-full";
import "@elements/password-strength";

import "@elements/titles/underlined-title";
import "@elements/titles/subtitle";

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
    country: string,
    username: string,
    state:string,
    noaccount:string,
    helpertext:string,
    errormessage: string,
    instruction: boolean,
    error: boolean,
    checkbox_label:string,
    subtitle:string,
    subtitletwo:string,
    dropdownicon:string,
    city:string,
    school:string,
    checkbox_label_two: string
  }

  const DEFAULT_ARGS:LoginArgs = {
    title: "Sign up - step 2",
    country: "Country",
    username: "Preferred language of communication*",
    state: "State",
    noaccount: "Already have an account?",
    helpertext: "Your password looks good", 
    errormessage: "",
    instruction: false,
    error: false,
    color: "red",
    checkbox_label:"I have read the terms and conditions (legal text…)",
    subtitle:"Tell us more about yourself so that we can tailor ",
    subtitletwo: "the content according to your specific needs",
    dropdownicon:"icn-chevron-dropdown-up.svg",
    city: "City",
    school: "School/Organization*",
    checkbox_label_two:"I would like to receive educational resources (GDPR legal text….)"
  }

export const SignUpTwo = (props?:LoginArgs) => {

    const {title,color,subtitle,dropdownicon,city,school,checkbox_label_two, subtitletwo,checkbox_label, country, username, state, noaccount, helpertext,errormessage, instruction, error} = props || DEFAULT_ARGS;


    return `
    <signup-full title="${title}">
        
        <sub-title slot="subtitle" title="${subtitle}"></sub-title>
        <sub-title slot="subtitle" title="${subtitletwo}"></sub-title>
        <dropdown-select slot="topleft" path="${dropdownicon}" label="${country}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <dropdown-select slot="topright" path="${dropdownicon}" label="${state}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <dropdown-select slot="bottomleft" path="${dropdownicon}" label="${city}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <dropdown-select slot="bottomright" path="${dropdownicon}" label="${school}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        
        <input-text slot="username" label="${username}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <input-checkbox slot="checkbox" label="${checkbox_label}"></input-checkbox>
        <input-checkbox slot="checkbox" label="${checkbox_label_two}"></input-checkbox>

        <div slot="submit">${RectangleButton()}</div>
        <plain-blue title="${noaccount}" slot="noaccount"></plain-blue>

        </signup-full>

    `
}

SignUpTwo.args = DEFAULT_ARGS;