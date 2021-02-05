import {argsToAttrs} from "@utils/attributes";
import "@elements/core/menu/ellipses-menu";

export default {
    title: "Core / Menu "
}

interface Args {
    clicked:boolean,
}

const DEFAULT_ARGS:Args = {
    clicked:true
}

export const EllipsesMenu = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {clicked} = props

    return `<ellipses-menu ${argsToAttrs(props)} ${clicked && "clicked"}></ellipses-menu>`;
}

EllipsesMenu.args = DEFAULT_ARGS;