import "@elements/entry/jig/publish/page";
import "@elements/entry/jig/publish/sharing-caring";
import "@elements/entry/home/TOSORT/image-thumbnail"; 
import "@elements/core/inputs/textarea";
import "@elements/core/inputs/switch";
import "@elements/entry/jig/publish/resources";
import "@elements/core/titles/ji";
import "@elements/widgets/tags/icon";
import { Rectangle } from "~/components/core/buttons/rectangle";
import { Top } from "~/components/widgets/tooltips/tooltip-top";
import { PillClose } from "~/components/core/pills/pill-close";

import { colorStyles } from "@elements/_styles/colors";

export default {
  title: 'Entry/Jig/Publish/Pages',
}

  
  interface PublishArgs {
        uploaded:boolean,
        sharing:boolean
    }

    const DEFAULT_ARGS:PublishArgs = {
      uploaded:false,
      sharing:false,
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
const STR_RESOURCES = "Additional resources (Optional)";
const STR_CATEGORIES = "Categories";
const STR_HELP = "Test";
const STR_JANE = "Jane Doe";
const STR_SELECTLANGUAGE = "Select language";
const STR_AGEPLACEHOLDER = "Select age";
const STR_SELECTLIST = "Select from the list";
const STR_SELECTCATEGORIES ="Select from the categories";
const STR_TOOLTIP ="Please fill the missing fields";
export const PublishFullOne = (props?:PublishArgs) => {

 const {sharing} = props || DEFAULT_ARGS;


    return `
    <publish-page>
        <image-thumbnail path="${STR_IMGTHUMBNAIL}" slot="column_one"></image-thumbnail>
        <input-switch slot="column_one" label="${STR_SLIDERLABEL}"></input-switch>
        <sharing-caring slot="sharingcaring" ${sharing && "sharing"}></sharing-caring>
        <input-text slot="column_two" mode="text" label="${STR_NAME}" helpertext="${STR_HELP}">
        </input-text>
        <input-textarea label="${STR_DESCRIPTION}" slot="column_two" placeholder="${STR_JANE}"></input-textarea>
        <dropdown-select slot="column_three" placeholder="${STR_SELECTLANGUAGE}" label="${STR_LANGUAGE}"  >
        </dropdown-select>
        <dropdown-select slot="column_three" label="${STR_AGE}" placeholder="${STR_AGEPLACEHOLDER}" >
        </dropdown-select>
        <dropdown-select slot="column_three" placeholder="${STR_SELECTLIST}"  label="${STR_GOAL}" >
        </dropdown-select>
        <dropdown-select slot="column_three" placeholder="${STR_SELECTCATEGORIES}" label="${STR_CATEGORIES}" >
        </dropdown-select>
        <div slot="pills">${PillClose({contents:STR_PILL})}</div>
        <div slot="pills">${PillClose({contents:STR_PILL})}</div>
        <div slot="pills">${PillClose({contents:STR_PILL})}</div>
        <div slot="pills">${PillClose({contents:STR_PILL})}</div>
        <div slot="pills">${PillClose({contents:STR_PILL})}</div>
        <title-ji slot="column_four" color="blue" size="medium" weight="normal">${STR_RESOURCES}</title-ji>
        <resources-column slot="column_four"></resources-column>
        <div slot="button">${Rectangle({contents:STR_BTNLABEL, size:"medium",color:"red", bold:false,italic:false,})}</div>
        <div slot="tooltip">${Top({contents:STR_TOOLTIP})}</div>
        </publish-page>
    
    `
}

PublishFullOne.args = DEFAULT_ARGS;

