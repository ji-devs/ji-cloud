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
        helpertext:string,
        errormessage: string,
        instruction: boolean,
        error: boolean,
        bold: boolean,
        uploaded:boolean,
        hidden:boolean,
        imghidden:boolean,
        errorwrapper:boolean
    
    }

    const DEFAULT_ARGS:PublishArgs = {
        helpertext: "", 
        errormessage: "",
        instruction: false,
        error: false,
        bold: false,
        uploaded:true,
        hidden:true,
        imghidden: true,
        errorwrapper: true,
      
      }

const STR_TITLE ="Settings and JIG info.";
const STR_SUBTITLE = "Last step before publishing";
const STR_BTNLABEL = "Publish JIG";
const STR_IMGTHUMBNAIL = "red-sea-book.png";
const STR_SLIDERLABEL = "My JIG is public";
const STR_NAME = "JIG’s name";
const STR_LANGUAGE = "Language of instructions";
const STR_DESCRIPTION = "Description";
const STR_MEDIUM = "medium";
const STR_RED = "red";
const STR_AGE = "Age";
const STR_GOAL = "Teachig Goal";
const STR_PILL = "School";
const STR_CHECKBOX = "Icn_CheckMark.svg";
const STR_ICONLABEL = "Test";
const STR_RESOURCES = "Additional resources (Optional)";
const STR_CATEGORIES = "Categories";
const STR_ADD = "Icn_Add.svg";
const STR_ICONLABELTWO = "Add Curriculum"


export const PublishFullOne = (props?:PublishArgs) => {

 const {uploaded, helpertext, errormessage,error, instruction, imghidden, errorwrapper} = props || DEFAULT_ARGS;


    return `
    <publish-full title="${STR_TITLE}" subtitle="${STR_SUBTITLE}">
        <image-thumbnail path="${STR_IMGTHUMBNAIL}" slot="column_one"></image-thumbnail>
        <slider-checkbox slot="column_one" label="${STR_SLIDERLABEL}"></slider-checkbox>
        <input-text slot="column_two" label="${STR_NAME}" helpertext="${helpertext}" ${errorwrapper && "errorwrapper"} error="${errormessage}" ${instruction && "instruction"} ${error && "error"} ${imghidden && "imghidden"}>
        </input-text>
        <textarea-text label="${STR_DESCRIPTION}" slot="column_two"></textarea-text>
        <dropdown-select slot="column_three" label="${STR_LANGUAGE}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <spacer-fourty slot="column_three"></spacer-fourty>
        <dropdown-select slot="column_three" label="${STR_AGE}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <spacer-fourty slot="column_three"></spacer-fourty>
        <dropdown-select slot="column_three"  label="${STR_GOAL}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <spacer-fourty slot="column_three"></spacer-fourty>
        <dropdown-select slot="column_three" label="${STR_CATEGORIES}" helpertext="${helpertext}" error="${errormessage}" ${instruction && "instruction"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <div slot="column_three">${PillListItem({label:STR_PILL})}</div>
        <plain-blue title="${STR_RESOURCES}" slot="column_four"></plain-blue>  
        <title-wicon title="${STR_ICONLABEL}" path="${STR_CHECKBOX}" ${uploaded && "uploaded"} slot="column_four"></title-wicon>
        <title-wicon title="${STR_ICONLABELTWO}" path="${STR_ADD}"  slot="column_four"></title-wicon>

        <div slot="button">${RectangleButton({label:STR_BTNLABEL, size:STR_MEDIUM,color:STR_RED, imglefthidden:true, imgrighthidden:true})}</div>
        <div slot="tooltip">${TooltipTop()}</div>
        </publish-full>
    
    `
}

PublishFullOne.args = DEFAULT_ARGS;

