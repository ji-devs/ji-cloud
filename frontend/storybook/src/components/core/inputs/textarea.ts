import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/textarea";

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

export const TextArea = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    const {width, ...textProps} = props

    return `
        <div style="width:${width}px">
            <input-textarea ${argsToAttrs(textProps)}></input-textarea>
        </div>
    `;
}

TextArea.args = DEFAULT_ARGS;