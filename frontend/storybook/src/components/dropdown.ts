import "@elements/dropdowns/selected-dropdown";
import "@elements/dropdowns/tree-dropdown";
import "@elements/dropdowns/tree-dropdown-child";
import "@elements/inputs/dropdown";
import "@elements/lists/list-hover";
import {ListHover} from "~/components/lists/school";
import { Dropdown, InputUnderlined } from "./input";
import "@elements/inputs/checkbox";

export default {
  title: 'Dropdown',
}


interface InputTextProps {
  open:boolean,
  label:string,
  width:number,
  checked:boolean
}

const DEFAULT_INPUT_TEXT_ARGS:InputTextProps = {
 open:true,
 label:"Language of instructions",
 width:300,
 checked:true
}

export const SelectedDropdown = ({label,width,checked, open}) => {
    return `
    <div style="width:${width}px">
    <dropdown-select ${open && "open"} label="${label}">
      <list-hover slot="inner-dropdown" ${checked && 'checked'}></list-hover>
    </dropdown-select>
    </div>
`
}

export const TreeDropdown = ({label, path}) => {
  return `<tree-dropdown label="${label}" path="${path}">
    <tree-dropdown-child></tree-dropdown-child>
  </tree-dropdown>
`
}


SelectedDropdown.args = DEFAULT_INPUT_TEXT_ARGS;
TreeDropdown.args = {
  label: "Category",
  path: "/icon-chevron-categories-24-px.svg"
}
