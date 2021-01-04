import "@elements/admin/templates-layout/image-label-full";
import {LeftLabel} from "~/components/admin/images/image-label-left";
import {LabelRight} from "~/components/admin/images/image-label-right";
import "@elements/titles/underlined-title";

export default {
  title: 'Full Pages/Image Label',
}


export const ImageLabelFullOne = ({title}) => {
    return `
    <imagelabel-full>
      <underlined-title slot="title" title=${title}></underlined-title>
      <div slot="left">${LeftLabel()}</div>
      <div slot="middle">${LabelRight()}</div>
    </imagelabel-full>
    
    `
}

ImageLabelFullOne.args = {
 title: "Label Images",
}

