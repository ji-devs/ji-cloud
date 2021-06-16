import {argsToAttrs} from "@utils/attributes";
import {HeaderController} from "~/components/module/_common/edit/widgets/header-controller";

import "@elements/module/_common/edit/header";
import "@elements/module/_groups/cards/edit/header/button-add";
export default {
    title: "Module / _GROUPS / Cards / Edit"
}


interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Header = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<module-header slot="header">
    ${HeaderController()}
    <header-button-add slot="button"></header-button-add>
    </module-header>`
}

Header.args = DEFAULT_ARGS;
