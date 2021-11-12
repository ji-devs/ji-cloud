import { argsToAttrs } from "@utils/attributes";
import "@elements/generic/buttons/flex";

export default {
    title: "Generic / Buttons",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Flex = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    //The content is just local / proof-of-concept
    //usually button-flex isn't used directly
    //rather, ButtonFlex is subclassed
    return `
    <button-flex ${argsToAttrs(props)}>
        <div style="background-color: red; width: 50px; height: 50px; display: flex; justify-content: center; align-items: center;">One</div>
        <div style="background-color: blue; color: white;">Two</div>
    </button-flex>`;
};

Flex.args = DEFAULT_ARGS;
