import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/inputs/primitives/base-select";
import "@elements/core/lists/li-check";

export default {
    title: "Core / Inputs / Primitives"
}

interface Args {
    label: string,
    value: string,
    placeholder: string,
    open: boolean,
    //Just used for demo
    count: number,
    // width: number,
}

const DEFAULT_ARGS:Args = {
    label: "Label",
    value: "",
    placeholder: "Select something",
    open: true,
    count: 10,
    // width: 300,
}

export const Select = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {count, ...dropdownProps} = props

    return `
        <div>
            <input-base-select ${argsToAttrs(dropdownProps)}>
                ${mapToString(arrayCount(count), i => {
                    return `<li-check>item ${i}</li-check>`;
                })}
            </input-base-select>
        </div>
    `;
}

Select.args = DEFAULT_ARGS;


export const SelectNested = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {count, ...dropdownProps} = props

    return `
        <div>
            <input-base-select nested ${argsToAttrs(dropdownProps)}>
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
            </input-base-select>
        </div>
    `;
}

SelectNested.args = DEFAULT_ARGS;
