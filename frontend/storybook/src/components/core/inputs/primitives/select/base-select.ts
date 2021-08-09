import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/primitives/select/base-select";
import { arrayCount, mapToString } from "@utils/array";

export default {
    title: "Core / Inputs / Primitives / Select"
}

interface Args {
    multiple: boolean,
}

const DEFAULT_ARGS: Args = {
    multiple: false,
}

export const BaseSelect = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <style>
            base-select:focus {
                background-color: palegoldenrod;
                border: solid green 2px;
            }

            base-option-group[active-within] [slot=anchor] {
                background-color: gray;
            }

            base-option[selected] {
                background-color: #ff00003d;
            }

            base-option[active],
            base-option-group[active] > [slot=anchor] {
                outline: solid black 2px;
            } 
        </style>
        <div style="padding: 40px;">
            <base-select ${argsToAttrs(props)}>
                <span slot="anchor">Label</span>
                <base-option>Option 1</base-option>
                <base-option selected>Option 2</base-option>
                <base-option>Option 3</base-option>
                <base-option>Option 4</base-option>
                <base-option-group>
                    <span slot="anchor">Option 5</span>
                    <base-option>Option 5.1</base-option>
                    <base-option>Option 5.2</base-option>
                    <base-option>Option 5.3</base-option>
                    <base-option-group>
                        <span slot="anchor">Option 5.4</span>
                        <base-option>Option 5.4.1</base-option>
                        <base-option>Option 5.4.2</base-option>
                        <base-option>Option 5.4.3</base-option>
                        <base-option>Option 5.4.4</base-option>
                        <base-option>Option 5.4.5</base-option>
                    </base-option-group>
                    <base-option>Option 5.5</base-option>
                    <base-option>Option 5.6</base-option>
                </base-option-group>
                <base-option>Option 6</base-option>
                <base-option>Option 7</base-option>
                <base-option-group>
                    <span slot="anchor">Option 8</span>
                    <base-option>Option 8.1</base-option>
                    <base-option>Option 8.2</base-option>
                    <base-option>Option 8.3</base-option>
                </base-option-group>
            </base-select>
        </div>
    `;
}

BaseSelect.args = DEFAULT_ARGS;
