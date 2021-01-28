import "@elements/entry/admin/images/meta/page";
import {LeftLabel} from "~/components/entry/admin/images/meta/image-label-left";
import "@elements/core/cards/blue";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/titles/variants/plain-black-list"; 
import "@elements/core/titles/ji";
import { Rectangle } from "~/components/core/buttons/rectangle";


export default {
  title: 'Admin/Image Label',
}

const STR_CATEGORIES = "Categories Summary";
const STR_GENERAL = "General Summary";
const STR_IMAGENAME = "Image name";
const STR_IMGCONTENT = "Moses parts the Nile";
const STR_DESCRIPTIONTITLE = "Image description";
const STR_DESCRIPTION = "An open book, Moses hold his stick and raise his hands up, and part the Nile. An open book, Moses hold his stick and raise his hands up, and part the Nile. An open book, Moses hold his stick and raise his hands up, and part the Nile.   "
const STR_STYLETITLE = "Image style";
const STR_CLIPART = "Clipart";
const STR_DRAWING = "Drawing";
const STR_COMIC = "Comic";
const STR_USED = "To be used only for";
const STR_CHABAD = "Chabad";
const STR_AGE = "Suitable for age";
const STR_ALL = "All ages";
const STR_STREAM = "Suitable for jewish stream";
const STR_NO = " No religion";
const STR_REFORM = "Reform/Conservative";
const STR_ORTHODOX = "Orthodox";
const STR_RED = "red";
const STR_MEDIUM = "medium";
const STR_NEXT = "Next"

export const ImageLabelFullThree = ({title}) => {
    return `
    <image-meta-page>
      <underlined-title slot="title" title=${title}></underlined-title>
      <div slot="left">${LeftLabel()}</div>
      <div slot="middle">
        <title-ji color="blue">${STR_GENERAL}</title-ji>
        <card-blue>
            <title-ji color="blue" weight="normal">${STR_IMAGENAME}</title-ji>
            <title-ji color="black" >${STR_IMGCONTENT}</title-ji>

            <title-ji color="blue">${STR_DESCRIPTIONTITLE}</title-ji>
            <title-ji color="black" weight="standard">${STR_DESCRIPTION}</title-ji>

            <title-ji color="blue">${STR_STYLETITLE}</title-ji>
            <div>
                <title-ji  color="black" weight="standard">${STR_CLIPART}</title-ji>
                <title-ji  color="black" weight="standard">${STR_DRAWING}</title-ji>
                <title-ji  color="black" weight="standard">${STR_COMIC}</title-ji>
           </div>

            <title-ji color="blue">${STR_USED}</title-ji>
            <title-ji color="black" weight="standard">${STR_CHABAD}</title-ji>

            <title-ji color="blue">${STR_AGE}</title-ji>
            <title-ji color="black" weight="standard">${STR_ALL}</title-ji>

            <title-ji color="blue">${STR_STREAM}</title-ji>
            <div>
                <title-ji color="black" weight="standard">${STR_NO}</title-ji>
                <title-ji color="black" weight="standard">${STR_REFORM}</title-ji>
                <title-ji color="black" weight="standard">${STR_ORTHODOX}</title-ji>
            </div>

        </card-blue>
      </div>
      <div slot="right">
      <title-ji color="blue" bold=true>${STR_CATEGORIES}</title-ji>
      <card-blue>
        
      </card-blue>
    </div>
    <div slot="button">
    ${Rectangle({color:STR_RED,size:STR_MEDIUM,contents:STR_NEXT,bold:false, italic:false})}
    </div>
    </image-meta-page>
    
    `
}

ImageLabelFullThree.args = {
 title: "Label Images",
}
