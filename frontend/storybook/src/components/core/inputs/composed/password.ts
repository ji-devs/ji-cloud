import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/composed/password";


export default {
    title: "Core / Inputs / Composed"
}

interface Args {
    error: boolean,
    label: string,
    hint: string,
    value: string,
    width: number,
}

const DEFAULT_ARGS: Args = {
    error: false,
    label: "Title",
    hint: "Minimum 8 digits, Must include a number",
    value: "hello",
    width: 300

}

export const Password = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;
    const { width, ...textProps } = props

    return `
        <div style="width:${width}px">
            <input-password ${argsToAttrs(textProps)}></input-password>
        </div>
    `;
}

Password.args = DEFAULT_ARGS;
