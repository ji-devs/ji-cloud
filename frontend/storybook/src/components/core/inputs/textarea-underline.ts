import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/textarea-underline";

export default {
    title: "Core / Inputs"
}

interface Args {
    label: string,
    placeholder: string,
    value: string,
    width:number,
    rows:number,
}

const DEFAULT_ARGS:Args = {
    label: "hello",
    placeholder: "Jane Doe",
    value: "world",
    width: 300,
    rows: 10,
}

export const TextAreaUnderline = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    const {width, ...textProps} = props

    return `
        <div style="width:${width}px">
            <input-textarea-underline ${argsToAttrs(textProps)}></input-textarea-underline>
        </div>
    `;
}

TextAreaUnderline.args = DEFAULT_ARGS;