import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/form/textarea";

export default {
    title: "Core / Inputs / Form"
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
            <input-form-textarea ${argsToAttrs(textProps)}></input-form-textarea>
        </div>
    `;
}

TextArea.args = DEFAULT_ARGS;
