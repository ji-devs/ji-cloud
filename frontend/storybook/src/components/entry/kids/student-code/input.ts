import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/kids/student-code/input";

export default {
    title: "Entry / Kids / Student Code",
};

interface Args {
    error: boolean;
}

const DEFAULT_ARGS: Args = {
    error: false,
};

export const Input = (props?: Args) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <kids-student-code-input slot="input" ${argsToAttrs(
            props
        )}></kids-student-code-input>
    `;
};

Input.args = DEFAULT_ARGS;
