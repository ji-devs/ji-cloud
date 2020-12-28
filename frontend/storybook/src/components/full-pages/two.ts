import "@elements/admin/templates-layout/image-label-full";
import {LeftLabel} from "~/components/admin/images/image-label-left";
import {TreeDropdown} from "~/components/dropdown";
import "@elements/titles/underlined-title";
import "@elements/cards/blue-card";
export default {
  title: 'Full Pages/Image Label',
}


export const ImageLabelFullTwo = ({title, label, path}) => {
    return `
    <imagelabel-full>
      <underlined-title slot="title" title=${title}></underlined-title>
      <div slot="left">${LeftLabel()}</div>
      <div slot="middle"><tree-dropdown label="${label}" path="${path}"></tree-dropdown></div>
      <div slot="right">
        <blue-card></blue-card>
      </div>
    </imagelabel-full>
    
    `
}

ImageLabelFullTwo.args = {
 title: "Label Images",
 label: "Category",
 path: "/icon-chevron-categories-24-px.svg"
}
