import { argsToAttrs } from "@utils/attributes";
import "@elements/core/hebrew-buttons/hebrew-keyboard/hebrew-keyboard";

export default {
    title: "Core / Hebrew buttons",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const HebrewKeyboard = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <input placeholder="Click here before typing">
        <hebrew-keyboard></hebrew-keyboard>
    `;
};

HebrewKeyboard.args = DEFAULT_ARGS;
