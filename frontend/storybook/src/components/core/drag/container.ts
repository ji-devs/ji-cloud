import {argsToAttrs} from "@utils/attributes";
import "@elements/core/drag/container";


export default {
    title: "Core / Drag"
}

interface Args {
    childHtml: string
}

const DEFAULT_ARGS:Args = {
    childHtml: ""
}

export const Container = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {childHtml, ...containerProps} = props;

    const renderChild = () => {
        if(childHtml == "") {
            return `
            <div style="width: 300px; height: 300px; background-color: beige; display: flex; align-items: center; justify-content: center">
            Content here 
            </div>`;
        } else {
            return childHtml
        }
    }
    return `<drag-container ${argsToAttrs(containerProps)}>
        ${renderChild()}
            </drag-container>`;
}

Container.args = DEFAULT_ARGS;
