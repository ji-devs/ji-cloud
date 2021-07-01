import {argsToAttrs} from "@utils/attributes";
import "@elements/core/inputs/composed/text-underline";

export default {
    title: "Core / Inputs / Composed"
}

interface Args {
    label: string,
    width:number,
}

const DEFAULT_ARGS:Args = {
    label: "hello",
    width: 300,
}

export const TextUnderline = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    const {width, ...textProps} = props

    return `
        <div style="width:${width}px">
            <input-text-underline ${argsToAttrs(textProps)}></input-text-underline>
        </div>
    `;
}

TextUnderline.args = DEFAULT_ARGS;