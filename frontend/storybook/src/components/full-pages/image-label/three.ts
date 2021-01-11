import "@elements/admin/templates-layout/image-label-full";
import {LeftLabel} from "~/components/admin/images/image-label-left";
import {LabelRight} from "~/components/admin/images/image-label-right";
import "@elements/titles/underlined-title";
import "@elements/titles/plain-black-list";
import "@elements/titles/plain-black";

export default {
  title: 'Full Pages/Image Label',
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


export const ImageLabelFullThree = ({title}) => {
    return `
    <imagelabel-full>
      <underlined-title slot="title" title=${title}></underlined-title>
      <div slot="left">${LeftLabel()}</div>
      <div slot="middle">
        <plain-blue title="${STR_GENERAL}"></plain-blue>
        <blue-card>
            <plain-blue title="${STR_IMAGENAME}" slot="content"></plain-blue>
            <plain-black title="${STR_IMGCONTENT}" slot="content"></plain-black>

            <plain-blue title="${STR_DESCRIPTIONTITLE}" slot="content"></plain-blue>
            <plain-black title="${STR_DESCRIPTION}" slot="content"></plain-black>

            <plain-blue title="${STR_STYLETITLE}" slot="content"></plain-blue>
            <plain-black-list slot="content">
                <plain-black title="${STR_CLIPART}"></plain-black>
                <plain-black title="${STR_DRAWING}"></plain-black>
                <plain-black title="${STR_COMIC}"></plain-black>
            </plain-black-list>

            <plain-blue title="${STR_USED}" slot="content"></plain-blue>
            <plain-black title="${STR_CHABAD}" slot="content"></plain-black>

            <plain-blue title="${STR_AGE}" slot="content"></plain-blue>
            <plain-black title="${STR_ALL}" slot="content"></plain-black>

            <plain-blue title="${STR_STREAM}" slot="content"></plain-blue>
            <plain-black-list slot="content">
                <plain-black title="${STR_NO}"></plain-black>
                <plain-black title="${STR_REFORM}"></plain-black>
                <plain-black title="${STR_ORTHODOX}"></plain-black>
            </plain-black-list>

        </blue-card>
      </div>
      <div slot="right">
      <plain-blue title="${STR_CATEGORIES}" bolder=true></plain-blue>
      <blue-card>
        
      </blue-card>
    </div>
    </imagelabel-full>
    
    `
}

ImageLabelFullThree.args = {
 title: "Label Images",
}
