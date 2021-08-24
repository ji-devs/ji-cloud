import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/kids/student-code/student-code";

export default {
    title: "Entry / Kids / Student Code"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const StudentCode = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <kids-student-code ${argsToAttrs(props)}>
            <kids-student-code-input slot="input"></kids-student-code-input>
            <kids-student-code-jigzi slot="jigzi" mode="default"></kids-student-code-jigzi>
        </kids-student-code>
    `;
}

StudentCode.args = DEFAULT_ARGS;
