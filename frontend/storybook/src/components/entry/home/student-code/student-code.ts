import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/home/student-code/student-code";

export default {
    title: "Entry / Home / Student Code"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const StudentCode = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <home-student-code ${argsToAttrs(props)}>
            <home-student-code-input slot="input"></home-student-code-input>
            <home-student-code-jigzi slot="jigzi" mode="default"></home-student-code-jigzi>
        </home-student-code>
    `;
}

StudentCode.args = DEFAULT_ARGS;
