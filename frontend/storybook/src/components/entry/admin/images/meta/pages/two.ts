import "@elements/entry/admin/images/meta/pages/page-two";
import { LeftLabel } from "~/components/entry/admin/images/meta/sections/image-label-left";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/titles/ji";
import "@elements/core/inputs/dropdowns/tree/tree";
import "@elements/core/cards/blue";
import {mockImagesHierarchy, TreeNode} from "~/mock/hierarchy";
import {DropdownTree} from "~/components/core/inputs/dropdowns/dropdown-tree"
import {ReportTree} from "~/components/core/reports/tree"
import {mapToString} from "@utils/array";
import { Rectangle } from "~/components/core/buttons/rectangle";


export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}

const STR_NEXT = "Next"



export const ImageMeta2 = () => {

  return `
    <image-meta-page-two>
      <div slot="left">${LeftLabel()}</div>
      
      <div slot="middle">${DropdownTree({mock: "images"})}</div>
      <div slot="right">${ReportTree({mock: "images"})}</div>
      <div slot="button">
      ${Rectangle({color:"red",size:"medium",contents:STR_NEXT,bold:false, italic:false})}
    </div>
    </image-meta-page-two>
    
    `
}