import "@elements/core/nav/steps-nav";
import "@elements/core/nav/step-nav";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Core / Nav",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const StepsNav = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <steps-nav ${argsToAttrs(props)}>
            <step-nav number="1" label="Themes" completed></step-nav>
            <step-nav number="2" label="Background" completed></step-nav>
            <step-nav number="3" label="Content" active></step-nav>
            <step-nav number="4" label="Preview"></step-nav>
        </steps-nav>
    `;
};

StepsNav.args = DEFAULT_ARGS;
