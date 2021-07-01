import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/composed/select";
import { arrayCount, mapToString } from "@utils/array";

export default {
    title: "Core / Inputs / Composed"
}

interface Args {
    label: string,
    value: string,
    placeholder: string,
    hint: string,
    error: boolean,
}

const DEFAULT_ARGS: Args = {
    label: "Hello",
    value: "",
    placeholder: "Placeholder",
    hint: "",
    error: false,
}

export const Select = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <input-select ${argsToAttrs(props)}>
            ${mapToString(arrayCount(10), i => {
                return `<li-check>item ${i}</li-check>`;
            })}
        </input-select>
    `;
}

Select.args = DEFAULT_ARGS;
