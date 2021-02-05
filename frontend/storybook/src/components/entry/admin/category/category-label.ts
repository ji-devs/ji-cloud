import "@elements/entry/admin/category/pages/category-label";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/titles/ji";
import "@elements/core/inputs/dropdowns/tree/tree-child";
import "@elements/core/inputs/dropdowns/tree/tree-static-child";
import "@elements/core/inputs/dropdowns/tree/tree";
import "@elements/core/cards/blue";
import "@elements/core/menu/ellipses-menu";
import {mockCategoryHierarchy, TreeNode} from "~/mock/hierarchy";

import {mapToString} from "@utils/array";
import { Rectangle } from "~/components/core/buttons/rectangle";


export default {
  title: 'Entry/Admin/Category/Pages',
}

const STR_PUBLISH = "Publish"


const leafNode = ({label, mode, open, children}:TreeNode) => {
  return `
    <dropdown-tree-child label="${label}" ${open ? "open" : ""} mode="${mode}">
    ${mapToString (children, leafNode)}
    <ellipses-menu slot="menu-dropdown">
      <category-dropdown></category-dropdown> 
    </ellipses-menu>
    </dropdown-tree-child>
  `;
}
const rootNode = ({label, open, children}:TreeNode) => {
  return `
    <dropdown-tree label="${label}" ${open ? "open" : ""}>
      ${mapToString (children, leafNode)}
      <ellipses-menu slot="menu-dropdown"></ellipses-menu>
    </dropdown-tree>
  `;
}



interface Props {
  data: Array<TreeNode>
}

const DEFAULT_ARGS:Props = {
  data: mockCategoryHierarchy
}

export const CategoryLabel = (props?:Props) => {
  const {data} = props || DEFAULT_ARGS;

  console.log(data);

  return `
    <category-label>
    
      <div slot="middle">
        ${mapToString(data, rootNode)}
        
      </div>
     
      <div slot="button">
      ${Rectangle({color:"red",size:"medium",contents:STR_PUBLISH,bold:false, italic:false})}
    </div>
    </category-label>
    
    `
}

CategoryLabel.args = DEFAULT_ARGS;
CategoryLabel.argTypes = {
  data: {
    control: {
      type: 'object',
    }
  }
}