import {argsToAttrs} from "@utils/attributes";
import "@elements/core/menu/kebab";


export default {
    title: "Core / Menu"
}

interface Args {
    visible:boolean,
    width: number,
}

const DEFAULT_ARGS:Args = {
    width: 300,
    visible: true,
}

export const Kebab = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {width, ...menuProps} = props;
    return `
    <menu-kebab ${argsToAttrs(menuProps)}>
        <div slot="menu-content">
            <div style="width: ${width}px">
            Menu Here
            </div>
        </div>
    </menu-kebab>
        `;
}

Kebab.args = DEFAULT_ARGS;
