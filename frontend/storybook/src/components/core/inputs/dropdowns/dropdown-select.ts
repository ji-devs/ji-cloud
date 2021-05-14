import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/inputs/dropdowns/dropdown-select";
import "@elements/core/lists/li-check";

export default {
    title: "Core / Inputs / Dropdowns"
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
    count: 7,
    width: 300,
}

export const DropdownSelect = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {count, width, ...dropdownProps} = props

    return `
        <div style="width: ${width}px">
            <dropdown-select ${argsToAttrs(dropdownProps)}>
                <li-check selected>Selected option</li-check>
                <li-check-collection>Collection</li-check-collection>
                <li-check-collection open>Open collection</li-check-collection>
                ${mapToString(arrayCount(count), i => {
                    return `<li-check>item ${i}</li-check>`;
                })}
            </dropdown-select>
        </div>
    `;
}

DropdownSelect.args = DEFAULT_ARGS;
