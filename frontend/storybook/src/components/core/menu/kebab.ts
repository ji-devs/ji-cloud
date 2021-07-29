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
    <div style="position: absolute; top: 100px; left: 100px;">
    <menu-kebab ${argsToAttrs(menuProps)}>
        <div>
            <div style="width: ${width}px">
            Menu Here
            </div>
        </div>
        </menu-kebab>
        </div>
        `;
}

Kebab.args = DEFAULT_ARGS;
