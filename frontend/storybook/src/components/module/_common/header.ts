import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/header";
import {HeaderController} from "./widgets/header-controller";

export default {
    title: "Module / _common"
}


interface Args {
    title: string
}

const DEFAULT_ARGS:Args = {
    title: "Module Title Here"
}

export const Header = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `<module-header slot="header" ${argsToAttrs(props)}>
    ${HeaderController()}
    </module-header>`
}

Header.args = DEFAULT_ARGS;
