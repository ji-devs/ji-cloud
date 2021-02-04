import "@elements/entry/admin/images/meta/pages/page-two";
import { LeftLabel } from "~/components/entry/admin/images/meta/sections/image-label-left";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/titles/ji";
import "@elements/core/inputs/dropdowns/tree/tree-child";
import "@elements/core/inputs/dropdowns/tree/tree-static-child";
import "@elements/core/inputs/dropdowns/tree/tree";
import "@elements/core/cards/blue";
import {mockImagesHierarchy, TreeNode} from "~/mock/hierarchy";

import {mapToString} from "@utils/array";
import { Rectangle } from "~/components/core/buttons/rectangle";


export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}

const STR_NEXT = "Next"


const leafNode = ({label, mode, open, children}:TreeNode) => {
  return `
    <dropdown-tree-child label="${label}" ${open ? "open" : ""} mode="${mode}">
    ${mapToString (children, leafNode)}
    </dropdown-tree-child>
  `;
}
const rootNode = ({label, open, children}:TreeNode) => {
  return `
    <dropdown-tree label="${label}" ${open ? "open" : ""}>
      ${mapToString (children, leafNode)}
    </dropdown-tree>
  `;
}

const rootNodeV2 = ({label, open, children}:TreeNode) => {
  return `
    <dropdown-tree-static label="${label}" ${open ? "open" : ""}>
      ${mapToString (children, leafNodeV2)}
    </dropdown-tree-static>
  `;
}
const leafNodeV2 = ({label, open, children}:TreeNode) => {
  return `
    <dropdown-tree-static-child label="${label}" ${open ? "open" : ""}>
    ${mapToString (children, leafNodeV2)}
    </dropdown-tree-static-child>
  `;
}

interface Props {
  data: Array<TreeNode>
}

const DEFAULT_ARGS:Props = {
  data: mockImagesHierarchy
}

export const ImageLabelFullTwo = (props?:Props) => {
  const {data} = props || DEFAULT_ARGS;

  console.log(data);

  return `
    <image-meta-page-two>
      <div slot="left">${LeftLabel()}</div>
      
      <div slot="middle">
        ${mapToString(data, rootNode)}
      </div>
      <div slot="right">
        
        
        <div>
        ${mapToString(data,rootNodeV2)}
        </div>
        
      </div>
      <div slot="button">
      ${Rectangle({color:"red",size:"medium",contents:STR_NEXT,bold:false, italic:false})}
    </div>
    </image-meta-page-two>
    
    `
}

ImageLabelFullTwo.args = DEFAULT_ARGS;
ImageLabelFullTwo.argTypes = {
  data: {
    control: {
      type: 'object',
    }
  }
}