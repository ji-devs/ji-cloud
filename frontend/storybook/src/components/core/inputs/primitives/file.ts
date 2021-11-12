import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/primitives/file";

export default {
    title: "Core / Inputs / Primitives",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const File = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        The input-file element comes with no styles, all styles here are inlined
        <input-file ${argsToAttrs(
            props
        )} style="height: 100px; width: 100px; border: dashed 3px gray; border-radius: 10px; place-content: center;">Add file</input-file>
    `;
};

File.args = DEFAULT_ARGS;
