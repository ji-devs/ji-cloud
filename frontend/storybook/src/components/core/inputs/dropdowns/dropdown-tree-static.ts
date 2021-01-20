import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import {mockHierarchy, TreeNode} from "~/mock/hierarchy";
import "@elements/core/inputs/dropdowns/tree/tree-static";
import "@elements/core/inputs/dropdowns/tree/tree-static-child";

export default {
    title: "Core / Inputs / Dropdowns"
}

interface Args {
    data: Array<TreeNode>,
}


const DEFAULT_ARGS:Args = {
    data: mockHierarchy
};

export const DropdownTreeStatic = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return mapToString(props.data, rootNode);
}

DropdownTreeStatic.args = DEFAULT_ARGS;
DropdownTreeStatic.argTypes = {
  data: {
    control: {
      type: 'object',
    }
  }
}
const leafNode = ({label, open, children}:TreeNode) => {
  return `
    <dropdown-tree-static-child label="${label}" ${open ? "open" : ""}>
    ${mapToString (children, leafNode)}
    </dropdown-tree-static-child>
  `;
}
const rootNode = ({label, open, children}:TreeNode) => {
  return `
    <dropdown-tree-static label="${label}" ${open ? "open" : ""}>
      ${mapToString (children, leafNode)}
    </dropdown-tree-static>
  `;
}