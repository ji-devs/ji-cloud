import "@elements/entry/admin/category/pages/category-two";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/titles/ji";
import "@elements/core/inputs/dropdowns/tree/tree-child";
import "@elements/core/inputs/dropdowns/tree/tree-static-child";
import "@elements/core/inputs/dropdowns/tree/tree";
import "@elements/core/cards/blue";
import {mockHierarchy, TreeNode} from "~/mock/hierarchy";

import {mapToString} from "@utils/array";
import { Rectangle } from "~/components/core/buttons/rectangle";


export default {
  title: 'Entry/Admin/Category/Pages',
}

const STR_PUBLISH = "Publish"


const leafNode = ({label, open, children}:TreeNode) => {
  return `
    <tree-dropdown-child label="${label}" ${open ? "open" : ""} page="category">
    ${mapToString (children, leafNode)}
    </tree-dropdown-child>
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
    <dropdown-tree-static-child label="${label}" ${open ? "open" : ""} page="category">
    ${mapToString (children, leafNodeV2)}
    </dropdown-tree-static-child>
  `;
}

interface Props {
  data: Array<TreeNode>
}

const DEFAULT_ARGS:Props = {
  data: mockHierarchy
}

export const CategoryLabelTwo = (props?:Props) => {
  const {data} = props || DEFAULT_ARGS;

  console.log(data);

  return `
    <category-two>
    
      <div slot="middle">
        ${mapToString(data, rootNode)}
      </div>
      <div slot="right">
        
        
        <div>
        ${mapToString(data,rootNodeV2)}
        </div>
        
      </div>
      <div slot="button">
      ${Rectangle({color:"red",size:"medium",contents:STR_PUBLISH,bold:false, italic:false})}
    </div>
    </category-two>
    
    `
}

CategoryLabelTwo.args = DEFAULT_ARGS;
CategoryLabelTwo.argTypes = {
  data: {
    control: {
      type: 'object',
    }
  }
}