import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/play/sections/ending";
import {mapToString, arrayIndex} from "@utils/array";


export default {
    title: "Module / Memory / Play / Sections"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Ending = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `
    <play-ending ${argsToAttrs(props)} slot="main">
    </play-ending>`;
}

Ending.args = DEFAULT_ARGS;
