import "@elements/entry/jig/publish/page";
import "@elements/entry/home/TOSORT/image-thumbnail"; 
import "@elements/core/inputs/textarea";
import "@elements/core/inputs/switch";
import "@elements/entry/jig/publish/resources";
import "@elements/core/titles/ji";
import "@elements/widgets/tags/icon";
import { Rectangle } from "~/components/core/buttons/rectangle";
import { Top } from "~/components/widgets/tooltips/tooltip-top";
import { PillClose } from "~/components/core/pills/pill-close";

export default {
  title: 'Entry/Jig/Publish/Pages',
}

  interface PublishArgs {
        errorname:string,
        errormessage: string,
        instruction: boolean,
        uploaded:boolean,
        errorwrapper:boolean
    
    }

    const DEFAULT_ARGS:PublishArgs = {
      errorname:"",
      errormessage: "",
      instruction: false,
      uploaded:false,
      errorwrapper: true,
      
      }


const STR_BTNLABEL = "Publish JIG";
const STR_IMGTHUMBNAIL = "red-sea-book.png";
const STR_SLIDERLABEL = "My JIG is public";
const STR_NAME = "JIG’s name";
const STR_LANGUAGE = "Language of instructions";
const STR_DESCRIPTION = "Description";
const STR_AGE = "Age";
const STR_GOAL = "Teaching Goal";
const STR_PILL = "School";
const STR_ERROR ="Please fill the missing fields";
const STR_RESOURCES = "Additional resources (Optional)";
const STR_CATEGORIES = "Categories";
const STR_HELP = "Test";
const STR_JANE = "Jane Doe";
const STR_SELECTLANGUAGE = "Select language";
const STR_AGEPLACEHOLDER = "Select age";
const STR_SELECTLIST = "Select from the list";
const STR_SELECTCATEGORIES ="Select from the categories";

export const PublishFullOne = (props?:PublishArgs) => {

 const {errormessage, instruction, errorwrapper, errorname} = props || DEFAULT_ARGS;


    return `
    <publish-page>
        <image-thumbnail path="${STR_IMGTHUMBNAIL}" slot="column_one"></image-thumbnail>
        <input-switch slot="column_one" label="${STR_SLIDERLABEL}"></input-switch>
        <input-text slot="column_two" mode="text" label="${STR_NAME}" helpertext="${STR_HELP}" error="${errorname}" ${instruction && "instruction"} ${errorwrapper && "errorwrapper"}>
        </input-text>
        <input-textarea label="${STR_DESCRIPTION}" slot="column_two" placeholder="${STR_JANE}"></input-textarea>
        <dropdown-select slot="column_three" placeholder="${STR_SELECTLANGUAGE}" label="${STR_LANGUAGE}"  error="${errormessage}" ${instruction && "instruction"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <dropdown-select slot="column_three" label="${STR_AGE}" placeholder="${STR_AGEPLACEHOLDER}" error="${errormessage}" ${instruction && "instruction"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <dropdown-select slot="column_three" placeholder="${STR_SELECTLIST}"  label="${STR_GOAL}" error="${errormessage}" ${instruction && "instruction"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <dropdown-select slot="column_three" placeholder="${STR_SELECTCATEGORIES}" label="${STR_CATEGORIES}" error="${errormessage}" ${instruction && "instruction"} ${errorwrapper && "errorwrapper"}>
        </dropdown-select>
        <div slot="pills">${PillClose({contents:STR_PILL})}</div>
        <div slot="pills">${PillClose({contents:STR_PILL})}</div>
        <div slot="pills">${PillClose({contents:STR_PILL})}</div>
        <div slot="pills">${PillClose({contents:STR_PILL})}</div>
        <div slot="pills">${PillClose({contents:STR_PILL})}</div>
        <title-ji slot="column_four" color="blue" size="medium" weight="normal">${STR_RESOURCES}</title-ji>
        <resources-column slot="column_four"></resources-column>
        <div slot="button">${Rectangle({contents:STR_BTNLABEL, size:"medium",color:"red", bold:false,italic:false,})}</div>
        <div slot="tooltip">${Top({contents:STR_ERROR})}</div>
        </publish-page>
    
    `
}

PublishFullOne.args = DEFAULT_ARGS;

