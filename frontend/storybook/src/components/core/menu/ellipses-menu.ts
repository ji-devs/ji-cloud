import {argsToAttrs} from "@utils/attributes";
import "@elements/core/menu/ellipses-menu";

export default {
    title: "Core / Menu "
}

interface Args {
    menuVisible:boolean,
}

const DEFAULT_ARGS:Args = {
    menuVisible:false
}

export const EllipsesMenu = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {menuVisible} = props

    return `<ellipses-menu ${argsToAttrs(props)} ${menuVisible && "menuVisible"}>
    <div>Hello</div></ellipses-menu>`;
}

EllipsesMenu.args = DEFAULT_ARGS;