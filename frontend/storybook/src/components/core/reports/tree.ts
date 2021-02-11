import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import {mockTempHierarchy, mockCategoryHierarchy, mockImagesHierarchy, TreeNode} from "~/mock/hierarchy";
import "@elements/core/reports/tree";
export default {
    title: "Core / Reports"
}

type Mock = "categories" | "images";
interface Args {
    mock: Mock
}


const DEFAULT_ARGS:Args = {
  mock: "categories"
};

export const ReportTree = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {mock} = props;

    const data = mock === "categories" ? mockCategoryHierarchy
      : mock === "images" ? mockImagesHierarchy
      : mockTempHierarchy;


    const renderNode = (nodeProps:TreeNode, depth: number) => {
      const {children, menuButton, menuContents, label, ...rest} = nodeProps;
      const hasChildren = children.length > 0;
      const isChild = depth > 0;


      const props = {
        hasChildren,
        isChild,
        ...rest
      } 

      return `
        <report-tree ${argsToAttrs(props)} >
           <span slot="content">${label}</span>
          <div slot="children">
            ${mapToString (children, child => renderNode(child, depth+1))}
          </div>
        </report-tree>
      `;
    }

    return mapToString(data, child => renderNode(child, 0));
}

ReportTree.args = DEFAULT_ARGS;
ReportTree.argTypes = {
  mock: {
    control: {
      type: 'inline-radio',
      options: ["categories", "images"]
    }
  }
}