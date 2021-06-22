import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/edit/widgets/hebrew-keyboard/hebrew-keyboard";

export default {
    title: "Module / _COMMON / edit / Widgets / Hebrew keyboard"
}


interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const HebrewKeyboard = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <input placeholder="Click here before typing">
        <hebrew-keyboard></hebrew-keyboard>
    `;
}

HebrewKeyboard.args = DEFAULT_ARGS;
