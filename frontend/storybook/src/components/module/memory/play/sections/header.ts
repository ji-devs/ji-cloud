import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/play/sections/header";
import {mapToString, arrayIndex} from "@utils/array";

export default {
    title: "Module / Memory / Play / Sections"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Header = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
    <play-header slot="header"></play-header>`;
}

Header.args = DEFAULT_ARGS;
