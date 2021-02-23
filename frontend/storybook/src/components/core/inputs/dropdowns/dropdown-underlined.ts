import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/inputs/dropdowns/dropdown-underlined";
import "@elements/core/lists/li-check";

export default {
    title: "Core / Inputs / Dropdowns"
}

interface Args {
    value: string,
    open: boolean,
    maxChildrenHeight: number,
    //Just used for demo
    count: number,
    width: number,
}

const DEFAULT_ARGS:Args = {
    value: "world",
    open: true,
    count: 3,
    maxChildrenHeight: 400,
    width: 300,
}

export const DropdownUnderlined = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {count, width, ...dropdownProps} = props

    return `
        <div style="width: ${width}px">
            <dropdown-underlined ${argsToAttrs(dropdownProps)}>
                ${mapToString(arrayCount(count), i => {
                    return `<li-check slot="options">item ${i}</li-check>`;
                })}
            </dropdown-underlined>
        </div>
    `;
}

DropdownUnderlined.args = DEFAULT_ARGS;
