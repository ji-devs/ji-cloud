import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/home/student-code/input";

export default {
    title: "Entry / Home / Student Code"
}

interface Args {
    error: boolean,
}

const DEFAULT_ARGS:Args = {
    error: false,
}

export const Input = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <home-student-code-input slot="input" ${argsToAttrs(props)}></home-student-code-input>
    `;
}

Input.args = DEFAULT_ARGS;
