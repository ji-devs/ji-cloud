import {argsToAttrs} from "@utils/attributes";
import "@elements/modules/tappingboard/menu-play";

export default {
    title: "Modules/Tappingboard"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const  MenuPlay = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<menu-play ${argsToAttrs(props)}></menu-play>`;
}

 MenuPlay.args = DEFAULT_ARGS;