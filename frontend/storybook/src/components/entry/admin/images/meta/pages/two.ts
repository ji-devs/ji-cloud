import "@elements/entry/admin/images/meta/page";
import { LeftLabel } from "~/components/entry/admin/images/meta/image-label-left";
import { DropdownTree } from "~/components/core/inputs/dropdowns/dropdown-tree";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/titles/ji";
import "@elements/core/inputs/dropdowns/tree/tree-child";
import "@elements/core/inputs/dropdowns/tree/tree-static-child";
import "@elements/core/inputs/dropdowns/tree/tree";
import "@elements/core/cards/blue";
import "@elements/core/titles/variants/title-w-icon";
import {mockHierarchy, TreeNode} from "~/mock/hierarchy";

import {mapToString} from "@utils/array";
import { Rectangle } from "~/components/core/buttons/rectangle";


export default {
  title: 'Admin/Image Label',
}

const STR_NEXT = "Next"


const leafNode = ({label, open, children}:TreeNode) => {
  return `
    <tree-dropdown-child label="${label}" ${open ? "open" : ""}>
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
    <dropdown-tree-static-child label="${label}" ${open ? "open" : ""}>
    ${mapToString (children, leafNodeV2)}
    </dropdown-tree-static-child>
  `;
}

interface Props {
  title: string,
  label: string,
  titletwo: string,
  data: Array<TreeNode>
}

const DEFAULT_ARGS:Props = {
  title: "Label Images",
  label: "Category",
  titletwo: "Categories Summary",
  data: mockHierarchy
}
//To put in search-input: "search-24-px.svg",
const STR_CATEGORIES = "Categories"

export const ImageLabelFullTwo = (props?:Props) => {
  const { title, label, data, titletwo } = props || DEFAULT_ARGS;

  console.log(data);

  return `
    <image-meta-page>
      <underlined-title slot="title" title=${title}></underlined-title>
      <div slot="left">${LeftLabel()}</div>
      
      <div slot="middle">
      <title-ji color="blue">${STR_CATEGORIES}</title-ji>
          <search-input placeholder="${label}" slot="input">

          </search-input>
        </title-w-input>
        ${mapToString(data, rootNode)}
      </div>
      <div slot="right">
        <title-ji color="blue" >${titletwo}</title-ji>
        <card-blue>
        <div>
        ${mapToString(data,rootNodeV2)}
        </div>
        </card-blue>
      </div>
      <div slot="button">
      ${Rectangle({color:"red",size:"medium",contents:STR_NEXT,bold:false, italic:false})}
    </div>
    </image-meta-page>
    
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