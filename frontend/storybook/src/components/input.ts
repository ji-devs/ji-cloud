import "@elements/inputs/input-text";
import "@elements/buttons/rectangle-button";
import "@elements/inputs/input-underlined";
import "@elements/inputs/checkbox";

export default {
  title: 'Input Text',
}

export const InputText = ({label,helpertext, error, instruction,errormessage }) => {
    return `<input-text label="${label}" helpertext="${helpertext}" error="${errormessage} ${instruction && "instruction"} ${error && "error"} >
 
    </input-text>`
}

export const Checkbox = ({label}) => {
  return `<input-checkbox label="${label}">

  </input-checkbox>`
}

export const InputUnderlined = ({label}) => {
  return `<input-underlined label="${label}">

  </input-underlined>`
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