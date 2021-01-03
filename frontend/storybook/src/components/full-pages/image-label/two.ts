import "@elements/admin/templates-layout/image-label-full";
import {LeftLabel} from "~/components/admin/images/image-label-left";
import {TreeDropdown} from "~/components/dropdown";
import "@elements/titles/underlined-title";
import "@elements/titles/plain-blue";

import "@elements/cards/blue-card";

import {TitleWithInput} from "~/components/input";

export default {
  title: 'Full Pages/Image Label',
}


export const ImageLabelFullTwo = ({title, label, path, icon, titletwo}) => {
    return `
    <imagelabel-full>
      <underlined-title slot="title" title=${title}></underlined-title>
      <div slot="left">${LeftLabel()}</div>
      
      <div slot="middle">
      <title-winput title="${title}">
  <search-input placeholder="${label}" path="${icon}" slot="input">

  </search-input>
  </title-winput>
      <tree-dropdown label="${label}" path="${path}"></tree-dropdown></div>
      <div slot="right">
        <plain-blue title="${titletwo}"></plain-blue>
        <blue-card></blue-card>
      </div>
    </imagelabel-full>
    
    `
}

ImageLabelFullTwo.args = {
 title: "Label Images",
 label: "Category",
 path: "/icon-chevron-categories-24-px.svg",
 icon: "search-24-px.svg",
 titletwo: "Categories Summary"
}
