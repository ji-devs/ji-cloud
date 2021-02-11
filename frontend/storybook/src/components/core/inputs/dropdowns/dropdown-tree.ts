import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import {mockTempHierarchy, mockCategoryHierarchy, mockImagesHierarchy, TreeNode} from "~/mock/hierarchy";
import "@elements/core/inputs/dropdowns/tree/tree";
import { ContainerMode } from "@elements/core/inputs/dropdowns/tree/tree";
import "@elements/core/buttons/ellipses";
import "@elements/core/buttons/expand";
import "@elements/core/menu/ellipses/ellipses-menu-line";
export default {
    title: "Core / Inputs / Dropdowns"
}

type Mock = "categories" | "images";
interface Args {
    mock: Mock
}


const DEFAULT_ARGS:Args = {
  mock: "categories"
};

export const DropdownTree = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {mock} = props;

    const data = mock === "categories" ? mockCategoryHierarchy
      : mock === "images" ? mockImagesHierarchy
      : mockTempHierarchy;


    const containerMode = mock ===  "categories" ? "multi-color"
      : "none";

    const getContent = ({label, mode, expandAllButton}:Partial<TreeNode>) => {
      let content = (() => {
        switch(mode) {
          case "checkbox": return `<input-checkbox slot="content" label="${label}"></input-checkbox>`
          case "textDisplay": return `<span slot="content">${label}</span>`
          case "textInput": return `<input slot="content" style="text" value="${label}" />`
          default: return ``;
        }
      })();

      if(expandAllButton) {
        content += `<button-expand slot="content"></button-expand>`;
      }

      return content;
    }

    const renderMenu = (content:any, menuContents:boolean) => {
      return `<ellipses-menu-line slot="content" ${menuContents ? "visible" : ""} hover>
        ${content}
        <div slot="menu-content">
          Menu Here!
        </div>
      </ellipses-menu-line>`
    }

    const renderNode = (nodeProps:TreeNode, depth: number) => {
      const {children, menuButton, menuContents, ...rest} = nodeProps;
      const hasChildren = children.length > 0;
      const isChild = depth > 0;


      const props = {
        containerMode,
        hasChildren,
        isChild,
        ...rest
      } 
    
      const content = getContent(rest);

      return `
        <dropdown-tree ${argsToAttrs(props)} >
          ${menuButton ? renderMenu(content, menuContents) : content }
          <div slot="children">
            ${mapToString (children, child => renderNode(child, depth+1))}
          </div>
        </dropdown-tree>
      `;
    }

    return mapToString(data, child => renderNode(child, 0));
}

DropdownTree.args = DEFAULT_ARGS;
DropdownTree.argTypes = {
  mock: {
    control: {
      type: 'inline-radio',
      options: ["categories", "images"]
    }
  }
}