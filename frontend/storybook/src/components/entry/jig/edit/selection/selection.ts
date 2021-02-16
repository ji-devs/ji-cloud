import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/selection/selection";
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
    

    return `<jig-edit-selection>TODO</jig-edit-selection>
    `;
}

ModuleSelection.args = DEFAULT_ARGS;
