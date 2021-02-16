import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/sidebar";
import "@elements/entry/jig/edit/sidebar/header";
import {mapToString, arrayIndex} from "@utils/array";
import {WithModules as Sidebar} from "../sidebar/sidebar";

export default {
    title: "Entry / Jig / Edit / Pages"
}

interface Args {
    nModules: number
}

const DEFAULT_ARGS:Args = {
    nModules: 10
}

export const Landing = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    
    const {nModules} = props;

    return `
        ${Sidebar({
            nModules
        })}
    `;
}

Landing.args = DEFAULT_ARGS;
