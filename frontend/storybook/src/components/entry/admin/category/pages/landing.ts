import "@elements/entry/admin/category/pages/landing";
import "@elements/entry/admin/category/buttons/add";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/titles/ji";
import "@elements/core/inputs/dropdowns/tree/tree-child";
import "@elements/core/inputs/dropdowns/tree/tree-static-child";
import "@elements/core/inputs/dropdowns/tree/tree";
import "@elements/core/cards/blue";
import "@elements/core/menu/ellipses-menu";
import "@elements/core/buttons/expand";
import {mockCategoryHierarchy, TreeNode} from "~/mock/hierarchy";
import {argsToAttrs} from "@utils/attributes";
import {mapToString} from "@utils/array";
import { Rectangle } from "~/components/core/buttons/rectangle";


export default {
  title: 'Entry/Admin/Category/Pages',
}

const STR_PUBLISH = "Publish"



const leafNode = ({children, ...props}:TreeNode) => {
  const hasChildren = children.length > 0;

  return `
    <dropdown-tree-child ${argsToAttrs(props)} ${hasChildren && "hasChildren"}>
      <ellipses-menu slot="menu-dropdown">
        <category-dropdown></category-dropdown> 
      </ellipses-menu>
      ${mapToString (children, leafNode)}
    </dropdown-tree-child>
  `;
}
const rootNode = ({children, ...props}:TreeNode) => {
  const hasChildren = children.length > 0;
  return `
    <dropdown-tree ${argsToAttrs(props)} ${hasChildren && "hasChildren"}>
      <ellipses-menu slot="menu-dropdown"></ellipses-menu>
      ${mapToString (children, leafNode)}
    </dropdown-tree>
  `;
}


interface Props {
  data: Array<TreeNode>
}

const DEFAULT_ARGS:Props = {
  data: mockCategoryHierarchy
}

export const Landing = (props?:Props) => {
  
  const {data} = props || DEFAULT_ARGS;

  console.log(data);

  return `
    <category-label>
    
      <div slot="middle">
        ${mapToString(data, rootNode)}
        
      </div>
      <button-expand slot="expand"></button-expand>
      <category-button-add slot="add"></category-button-add>

    </category-label>
    
    `
}

Landing.args = DEFAULT_ARGS;
Landing.argTypes = {
  data: {
    control: {
      type: 'object',
    }
  }
}