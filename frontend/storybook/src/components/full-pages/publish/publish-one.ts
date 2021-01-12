import "@elements/admin/templates-layout/publish-full";
import "@elements/image-thumbnail";
import "@elements/inputs/textarea-text";
import "@elements/inputs/slider";
import "@elements/dividers/spacer-fourty";
import "@elements/titles/plain-blue";
import "@elements/titles/title-w-icon";
import { RectangleButton } from "~/components/rectangle-button";
import { TooltipTop } from "~/components/tooltip";
import { PillListItem } from "~/components/lists/pill";

import { colorStyles } from "@elements/_styles/colors";

export default {
  title: 'Full Pages/Publish',
}

  
  interface PublishArgs {
        title: string,
        subtitle: string,
        path: string,
        name: string,
        helpertext:string,
        errormessage: string,
        instruction: boolean,
        error: boolean,
        label:string,
        dropdownicon:string,
        language: string,
        age:string,
        goal:string,
        categories:string,
        title_two: string,
        bold: boolean,
        icontitle_one:string,
        path_two:string,
        label_button:string,
        color:string,
        size: string,
        uploaded:boolean,
        hidden:boolean,
        pill_label:string,
        slider_label:string,

     
  
    
    }

    const DEFAULT_ARGS:PublishArgs = {
        title: "Settings and JIG info.",
        subtitle: "Last step before publishing",
        path: "red-sea-book.png",
        name: "JIG’s name",
        helpertext: "", 
        errormessage: "",
        instruction: false,
        error: false,
        label: "Description",
        dropdownicon:"icn-chevron-dropdown-up.svg",
        language: "Language of instructions",
        age: "Age",
        goal: "Teaching Goal",
        categories: "Categories",
        title_two:"Additional resources (Optional)",
        bold: false,
        icontitle_one: "Test",
        path_two:"Icn_CheckMark.svg",
        label_button:"Publish JIG",
        size:"medium",
        color:"red",
        uploaded:false,
        hidden:true,
        pill_label:"school",
        slider_label:"My JIG is public"


      }



export const PublishFullOne = (props?:PublishArgs) => {

 const {title, title_two,uploaded, pill_label,slider_label, size, color,label_button,icontitle_one, path_two, subtitle, bold, path, dropdownicon, name, helpertext, errormessage,error, instruction, label, language, age, goal, categories } = props || DEFAULT_ARGS;


    return `
    <publish-full title="${title}" subtitle="${subtitle}">
        <image-thumbnail path="${path}" slot="column_one"></image-thumbnail>
        <slider-checkbox slot="column_one" label="${slider_label}"></slider-checkbox>
        <input-text slot="column_two" label="${name}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </input-text>
        <textarea-text label="${label}" slot="column_two"></textarea-text>
        <dropdown-select slot="column_three" path="${dropdownicon}" label="${language}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <spacer-fourty slot="column_three"></spacer-fourty>
        <dropdown-select slot="column_three" path="${dropdownicon}" label="${age}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <spacer-fourty slot="column_three"></spacer-fourty>
        <dropdown-select slot="column_three" path="${dropdownicon}" label="${goal}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <spacer-fourty slot="column_three"></spacer-fourty>
        <dropdown-select slot="column_three" path="${dropdownicon}" label="${categories}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${error && "error"} >
        </dropdown-select>
        <div slot="column_three">${PillListItem({label:pill_label})}</div>
        <plain-blue title="${title_two}" slot="column_four"></plain-blue>  
        <title-wicon title="${icontitle_one}" path="${path_two}" class="${uploaded && "uploaded"}" slot="column_four"></title-wicon>
        <div slot="button">${RectangleButton({label:label_button, size:size,color:color})}</div>
        <div slot="tooltip">${TooltipTop()}</div>
        </publish-full>
    
    `
}

PublishFullOne.args = DEFAULT_ARGS;

