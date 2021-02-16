import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/sidebar";
import "@elements/entry/jig/edit/sidebar/header";
import {mapToString, arrayIndex} from "@utils/array";

export default {
    title: "Entry / Jig / Edit / Selection"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const ModuleSelection = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    

    return `<div>TODO</div>
    `;
}

ModuleSelection.args = DEFAULT_ARGS;
