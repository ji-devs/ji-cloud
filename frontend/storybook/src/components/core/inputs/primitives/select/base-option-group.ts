import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/primitives/select/base-option-group";
import { arrayCount, mapToString } from "@utils/array";

export default {
    title: "Core / Inputs / Primitives / Select",
};

interface Args {
    open: boolean;
}

const DEFAULT_ARGS: Args = {
    open: true,
};

export const BaseOptionGroup = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <div style="padding: 40px;">
            <base-option-group>
                <span slot="anchor">Option 5</span>
                <base-option>Option 5.1</base-option>
                <base-option>Option 5.2</base-option>
                <base-option>Option 5.3</base-option>
                <base-option>Option 5.4</base-option>
                <base-option>Option 5.5</base-option>
            </base-option-group>
        </div>
    `;
};

BaseOptionGroup.args = DEFAULT_ARGS;
