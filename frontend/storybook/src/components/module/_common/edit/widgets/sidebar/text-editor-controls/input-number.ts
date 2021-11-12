import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/edit/widgets/text-editor-controls/input-number";

export default {
    title: "Module / _COMMON / edit / Widgets / Sidebar / Text Editor Controls",
};

interface Args {
    value: number;
}

const DEFAULT_ARGS: Args = {
    value: 3,
};

export const InputNumber = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <text-editor-controls-input-number ${argsToAttrs(
            props
        )}></text-editor-controls-input-number>
    `;
};

InputNumber.args = DEFAULT_ARGS;
