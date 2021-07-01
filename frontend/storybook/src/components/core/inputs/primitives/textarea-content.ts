import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/primitives/textarea-content";

export default {
    title: "Core / Inputs / Primitives"
}

interface Args {
  value:string,
  editing: boolean
  width:number,
  height:number,
}

const DEFAULT_ARGS:Args = {
    editing: true,
  value:"hello",
  width:300,
  height:300,
}

export const TextAreaContent = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {width, height, ...textProps} = props

    return `
        <div style="width:${width}px; height: ${height}px; background-color: beige;">
            <input-textarea-content ${argsToAttrs(textProps)}></input-textarea-content>
        </div>
    `;
}

TextAreaContent.args = DEFAULT_ARGS;
