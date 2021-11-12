import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/composed/select/select";
import "@elements/core/inputs/composed/select/option";
import "@elements/core/inputs/composed/select/option-group";
import { arrayCount, mapToString } from "@utils/array";

export default {
    title: "Core / Inputs / Composed",
};

interface Args {
    label: string;
    value: string;
    placeholder: string;
    error: boolean;
    hint: string;
    multiple: boolean;
}

const DEFAULT_ARGS: Args = {
    label: "Label",
    value: "",
    placeholder: "Placeholder",
    error: false,
    hint: "",
    multiple: false,
};

export const Select = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <div style="padding:40px; width:300px">
            <input-select ${argsToAttrs(props)}>
                <input-select-option>Option 1</input-select-option>
                <input-select-option selected>Option 2</input-select-option>
                <input-select-option>Option 3</input-select-option>
                <input-select-option>Option 4</input-select-option>
                <input-select-option-group label="option 5">
                    <input-select-option>Option 5.1</input-select-option>
                    <input-select-option>Option 5.2</input-select-option>
                    <input-select-option-group label="option 5.3">
                        <input-select-option>Option 5.3.1</input-select-option>
                        <input-select-option>Option 5.3.2</input-select-option>
                        <input-select-option>Option 5.3.3</input-select-option>
                        <input-select-option>Option 5.3.4</input-select-option>
                    </input-select-option-group>
                    <input-select-option>Option 5.4</input-select-option>
                </input-select-option-group>
                <input-select-option-group label="option 6">
                    <input-select-option>Option 6.1</input-select-option>
                    <input-select-option>Option 6.2</input-select-option>
                    <input-select-option>Option 6.3</input-select-option>
                    <input-select-option>Option 6.4</input-select-option>
                </input-select-option-group>
            </input-select>
        </div>
    `;
};

Select.args = DEFAULT_ARGS;
