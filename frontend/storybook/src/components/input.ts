import "@elements/inputs/input-text";
import "@elements/buttons/rectangle-button";
import "@elements/inputs/input-underlined";
import "@elements/inputs/textarea-underlined";
import "@elements/inputs/search";
import "@elements/inputs/dropdown";
import "@elements/inputs/title-winput";
import "@elements/inputs/checkbox";
import "@elements/inputs/add-image";

export default {
  title: 'Input Text',
}

export const InputText = ({label,helpertext, error, instruction,errormessage }) => {
    return `<input-text label="${label}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
 
    </input-text>`
    
}

export const SearchInput = ({label, path}) => {
  return `<search-input placeholder="${label}" path="${path}" >

  </search-input>`
  
}

export const Dropdown = ({label,helpertext, error, instruction,errormessage, errorwrapper, path, imghidden}) => {
  return `<dropdown-select label="${label}" helpertext="${helpertext}" error="${errormessage}" ${imghidden  && "imghidden"} ${errorwrapper && "errorwrapper"} ${instruction && "instruction"} ${error && "error"} >
 
  </dropdown-select>`
  
}

export const Checkbox = ({label}) => {
  return `<input-checkbox label="${label}">

  </input-checkbox>`
}

export const InputUnderlined = ({label}) => {
  return `<input-underlined label="${label}">

  </input-underlined>`
}

export const TextareaUnderlined = ({label}) => {
  return `<textarea-underlined label="${label}">

  </textarea-underlined>`
}

export const AddImage = () => {
  return `<add-image label="">

  </add-image>`
}

export const TitleWithInput = ({title,label, path}) => {
  return `<title-winput title="${title}">
  <search-input placeholder="${label}" path="${path}" slot="input">

  </search-input>
  </title-winput>`
}

InputText.args = {
 instruction:false,
 errormessage: "Wrong Password",
 label: "Title",
 helpertext: "Minimum 8 digits, Must include a number",
 error: true,

}

Checkbox.args = {
  label:"Placeholder",

}

InputUnderlined.args = {
  label:"First Name"
}

TextareaUnderlined.args = {
  label:"First Name"
}

SearchInput.args = {
  label:"Category Search",
  path:"search-24-px.svg",
}

TitleWithInput.args = {
  label:"Category",
  path:"search-24-px.svg",
  title:"Placeholder"
}

Dropdown.args = {
  instruction:false,
 errormessage: "Wrong Password",
 label: "Title",
 helpertext: "Minimum 8 digits, Must include a number",
 error: true,
 path:"icn-chevron-dropdown-up.svg"
}