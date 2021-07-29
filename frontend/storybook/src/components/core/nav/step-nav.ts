import {argsToAttrs} from "@utils/attributes";
import "@elements/core/nav/step-nav";

export default {
    title: "Core / Nav"
}

interface Args {
    active: boolean,
    completed: boolean,
    number: number,
    label: string,
}

const DEFAULT_ARGS:Args = {
    active: false,
    completed: false,
    number: 2,
    label: "Label here"
}

export const StepNav = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <step-nav ${argsToAttrs(props)}></step-nav>
    `;
}

StepNav.args = DEFAULT_ARGS;
