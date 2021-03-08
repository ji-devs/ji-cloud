import "@elements/entry/admin/category/pages/landing";
import "@elements/entry/admin/category/buttons/add";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/titles/ji";
import "@elements/core/inputs/dropdowns/tree/tree";
import "@elements/core/cards/blue";
import "@elements/core/buttons/expand";
import {mockCategoryHierarchy, TreeNode} from "~/mock/hierarchy";
import {argsToAttrs} from "@utils/attributes";
import {mapToString} from "@utils/array";
import { Rectangle } from "~/components/core/buttons/rectangle";
import {DropdownTree} from "~/components/core/inputs/dropdowns/dropdown-tree"

export default {
  title: 'Entry/Admin/Category/Pages',
}

const STR_PUBLISH = "Publish"

export const Landing = () => {
  
  return `
    <category-page>
    
      <div slot="middle">
        ${DropdownTree({mock: "categories"})}
      </div>
      <button-expand slot="expand"></button-expand>
      <category-button-add slot="add"></category-button-add>

    </category-label>
    
    `
}