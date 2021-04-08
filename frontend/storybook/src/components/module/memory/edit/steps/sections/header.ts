import {argsToAttrs} from "@utils/attributes";
import {HeaderController} from "~/components/module/_common/widgets/header-controller";

import "@elements/module/memory/edit/steps/sections/header/header";
import "@elements/module/memory/edit/steps/sections/header/button-add";
export default {
    title: "Module / Memory / Edit / Steps / Sections"
}


interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Header = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<header-memory slot="header">
    ${HeaderController()}
    <header-button-add slot="button"></header-button-add>
    </header-memory>`
}

Header.args = DEFAULT_ARGS;
