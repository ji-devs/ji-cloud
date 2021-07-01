import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/wrapper";
import { arrayCount, mapToString } from "@utils/array";

export default {
    title: "Core / Inputs / Wrappers"
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

export const Wrapper = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <input-wrapper ${argsToAttrs(props)}>
            <input ${argsToAttrs(props)}>
        </input-wrapper>
        <br>
        <input-wrapper ${argsToAttrs(props)}>
            <textarea ${argsToAttrs(props)}></textarea>
        </input-wrapper>
        <br>
        <input-wrapper ${argsToAttrs(props)}>
            <input-base-select value="value">
                ${mapToString(arrayCount(10), i => {
                    return `<li-check>item ${i}</li-check>`;
                })}
            </input-base-select>
        </input-wrapper>
    `;
}

Wrapper.args = DEFAULT_ARGS;
