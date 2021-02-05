import "@elements/entry/admin/images/meta/pages/page-three";
import {LeftLabel} from "~/components/entry/admin/images/meta/sections/image-label-left";
import "@elements/core/cards/blue";
import "@elements/core/titles/ji";
import { Rectangle } from "~/components/core/buttons/rectangle";


export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}



const STR_IMGCONTENT = "Moses parts the Nile";
const STR_DESCRIPTION = "An open book, Moses hold his stick and raise his hands up, and part the Nile. An open book, Moses hold his stick and raise his hands up, and part the Nile. An open book, Moses hold his stick and raise his hands up, and part the Nile.   "
const STR_CLIPART = "Clipart";
const STR_DRAWING = "Drawing";
const STR_COMIC = "Comic";
const STR_CHABAD = "Chabad";
const STR_ALL = "All ages";
const STR_NO = " No religion";
const STR_REFORM = "Reform/Conservative";
const STR_ORTHODOX = "Orthodox";
const STR_NEXT = "Next"

export const ImageLabelFullThree = () => {
    return `
    <image-meta-page-three>
      <div slot="left">${LeftLabel()}</div>
      <div slot="imagename">
      <title-ji color="black">${STR_IMGCONTENT}</title-ji>
      </div>
      <div slot="description">
      <title-ji color="black">${STR_DESCRIPTION}</title-ji>
      </div>
      <div slot="style">
      <title-ji color="black">${STR_CLIPART}</title-ji>
      <title-ji color="black">${STR_DRAWING}</title-ji>
      <title-ji color="black">${STR_COMIC}</title-ji>
      </div>
      <div slot="used">
      <title-ji color="black">${STR_CHABAD}</title-ji>
      </div>
      <div slot="age">
      <title-ji color="black">${STR_ALL}</title-ji>
      </div>
      <div slot="stream">
      <title-ji color="black">${STR_NO}</title-ji>
      <title-ji color="black">${STR_REFORM}</title-ji>
      <title-ji color="black">${STR_ORTHODOX}</title-ji>
      </div>
      <div slot="right">
      
      <card-blue>
        
      </card-blue>
      </div>
    <div slot="button">
    ${Rectangle({color:"red",size:"medium",contents:STR_NEXT,bold:false, italic:false})}
    </div>
    </image-meta-page-three>
    
    `
}


