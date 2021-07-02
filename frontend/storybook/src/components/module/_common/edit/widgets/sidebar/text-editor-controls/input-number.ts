import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/edit/widgets/text-editor-controls/input-number";

export default {
    title: "Module / _COMMON / edit / Widgets / Sidebar / Text Editor Controls"
}

interface Args {
    value: number,
    min: number,
    max: number,
}

const DEFAULT_ARGS: Args = {
    value: 3,
    min: -2,
    max: 4,
}

export const InputNumber = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <text-editor-controls-input-number ${argsToAttrs(props)}></text-editor-controls-input-number>
    `;
}

InputNumber.args = DEFAULT_ARGS;
