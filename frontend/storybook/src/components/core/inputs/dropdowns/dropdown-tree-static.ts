import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import {mockCategoryHierarchy, mockImagesHierarchy, TreeNode} from "~/mock/hierarchy";
import "@elements/core/inputs/dropdowns/tree/tree-static";
import "@elements/core/inputs/dropdowns/tree/tree-static-child";

export default {
    title: "Core / Inputs / Dropdowns"
}

interface Args {
    data: Array<TreeNode>,
}


const DEFAULT_ARGS_ONE:Args = {
    data: mockImagesHierarchy
};

export const DropdownTreeStaticOne = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS_ONE, ...props} : DEFAULT_ARGS_ONE;

    return mapToString(props.data, rootNode);
}

DropdownTreeStaticOne.args = DEFAULT_ARGS_ONE;
DropdownTreeStaticOne.argTypes = {
  data: {
    control: {
      type: 'object',
    }
  }
}

const DEFAULT_ARGS_TWO:Args = {
  data: mockCategoryHierarchy
};

export const DropdownTreeStaticTwo = (props?:Partial<Args>) => {
  props = props ? {...DEFAULT_ARGS_TWO, ...props} : DEFAULT_ARGS_TWO;

  return mapToString(props.data, rootNode);
}

DropdownTreeStaticTwo.args = DEFAULT_ARGS_TWO;
DropdownTreeStaticTwo.argTypes = {
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