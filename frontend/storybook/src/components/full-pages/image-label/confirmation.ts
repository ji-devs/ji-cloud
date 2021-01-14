import "@elements/admin/templates-layout/add-image-full";
import {LeftLabel} from "~/components/admin/images/image-label-left";
import {LabelRight} from "~/components/admin/images/image-label-right";
import "@elements/titles/underlined-title";
import "@elements/inputs/add-image";
import "@elements/tooltip/confirmation-popup";
import { RectangleButton } from "~/components/rectangle-button";

export default {
  title: 'Full Pages/Image Label',
}

const STR_LABEL = "Label Images";
const STR_BLUE = "blue";
const STR_MEDIUM = "medium";
const STR_UPLOAD = "Upload image";
const STR_PLUS = "icon-add-24.svg";
const STR_CONFIRMATION = "Image was published";
const STR_IMG = "red-sea-book.png";
const STR_ICONCHECK ="done-24-px-v.svg"

export const Confirmation = () => {
    return `
    <add-image-full>
      <underlined-title slot="title" title=${STR_LABEL}></underlined-title>
      <confirmation-popup label="${STR_CONFIRMATION}" path="${STR_IMG}" icon="${STR_ICONCHECK}" slot="title"></confirmation-popup>
      <div slot="left">
        <add-image>
            ${RectangleButton({color:STR_BLUE,size:STR_MEDIUM,label:STR_UPLOAD,imgrighthidden:true,iconpath:STR_PLUS,bold:false, italic:false,imglefthidden:false})}
        </add-image>
      </div>
      
    </add-image-full>
    
    `
}

Confirmation.args = {
 title: "Label Images",
}
