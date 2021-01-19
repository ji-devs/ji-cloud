import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import {mockHierarchy, TreeNode} from "~/mock/hierarchy";
import "@elements/core/inputs/dropdowns/tree/tree";
import "@elements/core/inputs/dropdowns/tree/tree-child";

export default {
    title: "Core / Inputs / Dropdowns"
}

interface Args {
    data: Array<TreeNode>,
}


const DEFAULT_ARGS:Args = {
    data: mockHierarchy
};

export const DropdownTree = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return mapToString(props.data, rootNode);
}

DropdownTree.args = DEFAULT_ARGS;
DropdownTree.argTypes = {
  data: {
    control: {
      type: 'object',
    }
  }
}
const leafNode = ({label, open, children}:TreeNode) => {
  return `
    <dropdown-tree-child label="${label}" ${open ? "open" : ""}>
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