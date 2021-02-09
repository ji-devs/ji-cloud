import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import {mockCategoryHierarchy, mockImagesHierarchy, TreeNode} from "~/mock/hierarchy";
import "@elements/core/inputs/dropdowns/tree/tree";
import "@elements/core/inputs/dropdowns/tree/tree-child";
import { Mode } from "@elements/core/inputs/dropdowns/tree/tree-child";

export default {
    title: "Core / Inputs / Dropdowns"
}

interface Args {
    data: Array<TreeNode>,
    
    
}


const DEFAULT_ARGS_ONE:Args = {
    data: mockImagesHierarchy,
 
};

export const DropdownTreeOne = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS_ONE, ...props} : DEFAULT_ARGS_ONE;

    return mapToString(props.data, rootNode);
}

DropdownTreeOne.args = DEFAULT_ARGS_ONE;
DropdownTreeOne.argTypes = {
  data: {
    control: {
      type: 'object',
    }
  }
}

const DEFAULT_ARGS_TWO:Args = {
  data: mockCategoryHierarchy,

};

export const DropdownTreeTwo = (props?:Partial<Args>) => {
  props = props ? {...DEFAULT_ARGS_TWO, ...props} : DEFAULT_ARGS_TWO;

  return mapToString(props.data, rootNode);
}

DropdownTreeTwo.args = DEFAULT_ARGS_TWO;
DropdownTreeTwo.argTypes = {
data: {
  control: {
    type: 'object',
  }
}
}

const leafNode = ({children, ...props}:TreeNode) => {
  const hasChildren = children.length > 0;

  return `
    <dropdown-tree-child ${argsToAttrs(props)} ${hasChildren && "hasChildren"}>
    ${mapToString (children, leafNode)}
    </dropdown-tree-child>
  `;
}
const rootNode = ({children, ...props}:TreeNode) => {
  const hasChildren = children.length > 0;
  return `
    <dropdown-tree ${argsToAttrs(props)} ${hasChildren && "hasChildren"}>
      ${mapToString (children, leafNode)}
    </dropdown-tree>
  `;
}