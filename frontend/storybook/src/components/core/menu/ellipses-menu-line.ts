import {argsToAttrs} from "@utils/attributes";
import "@elements/core/menu/ellipses/ellipses-menu-line";

export default {
    title: "Core / Menu"
}

interface Args {
    hover:boolean,
    visible:boolean,
}

const DEFAULT_ARGS:Args = {
    hover: true,
    visible: true
}

export const EllipsesMenuLine = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<ellipses-menu-line ${argsToAttrs(props)}>
        <div slot="content">Line Here</div>
        <div slot="menu-content">Menu Here</div>
    </ellipses-menu-line>`;
}

EllipsesMenuLine.args = DEFAULT_ARGS;