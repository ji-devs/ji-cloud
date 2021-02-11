import {argsToAttrs} from "@utils/attributes";
import "@elements/modules/tappingboard/menu-click";

export default {
    title: "Modules/Tappingboard"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const MenuClick = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<menu-click ${argsToAttrs(props)}></menu-click>`;
}

MenuClick.args = DEFAULT_ARGS;