import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/inputs/old/dropdown-select";
import "@elements/core/lists/li-check";

export default {
    title: "Core / Inputs / Old"
}

interface Args {
    label: string,
    value: string,
    placeholder: string,
    open: boolean,
    error: boolean,
    //Just used for demo
    count: number,
    width: number,
}

const DEFAULT_ARGS:Args = {
    label: "Label",
    value: "",
    placeholder: "Select something",
    open: true,
    error: true,
    count: 10,
    width: 300,
}

export const DropdownSelect = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {count, width, ...dropdownProps} = props

    return `
        <div style="width: ${width}px">
            <dropdown-select-old ${argsToAttrs(dropdownProps)}>
                ${mapToString(arrayCount(count), i => {
                    return `<li-check>item ${i}</li-check>`;
                })}
            </dropdown-select-old>
        </div>
    `;
}

DropdownSelect.args = DEFAULT_ARGS;


export const DropdownSelectNested = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {count, width, ...dropdownProps} = props

    return `
        <div style="width: ${width}px">
            <dropdown-select-old nested ${argsToAttrs(dropdownProps)}>
                <li-check selected>Selected option</li-check>
                <li-check-collection>
                    <span slot="label">Label</span>
                    ${mapToString(arrayCount(count), i => {
                        return `<li-check slot="options">item ${i}</li-check>`;
                    })}
                </li-check-collection>
                <li-check-collection open>
                    <span slot="label">Label</span>
                    ${mapToString(arrayCount(count), i => {
                        return `<li-check slot="options">item ${i}</li-check>`;
                    })}
                </li-check-collection>
            </dropdown-select-old>
        </div>
    `;
}

DropdownSelectNested.args = DEFAULT_ARGS;
