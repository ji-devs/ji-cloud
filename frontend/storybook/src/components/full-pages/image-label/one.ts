import "@elements/admin/templates-layout/image-label-full";
import {LeftLabel} from "~/components/admin/images/image-label-left";
import {LabelRight} from "~/components/admin/images/image-label-right";
import "@elements/titles/underlined-title";
import { RectangleButton } from "~/components/rectangle-button";


export default {
  title: 'Full Pages/Image Label',
}

const STR_RED = "red";
const STR_MEDIUM = "medium";
const STR_NEXT = "Next"

export const ImageLabelFullOne = ({title}) => {
    return `
    <imagelabel-full>
      <underlined-title slot="title" title=${title}></underlined-title>
      <div slot="left">${LeftLabel()}</div>
      <div slot="middle">${LabelRight()}</div>
      <div slot="button">
        ${RectangleButton({color:STR_RED,size:STR_MEDIUM,label:STR_NEXT,imgrighthidden:true,bold:false, italic:false,imglefthidden:true, iconpath:""})}
      </div>

    </imagelabel-full>
    
    `
}

ImageLabelFullOne.args = {
 title: "Label Images",
}

