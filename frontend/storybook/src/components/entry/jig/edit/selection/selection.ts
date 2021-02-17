import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/selection/selection";
import {mapToString, arrayIndex} from "@utils/array";
import {Card} from "./card";
import {ModuleKind, moduleKinds} from "@elements/entry/jig/module-types";
export default {
    title: "Entry / Jig / Edit / Selection"
}

interface Args {
    moduleKinds: Array<ModuleKind>
}

const DEFAULT_ARGS:Args = {
    moduleKinds: moduleKinds.filter(module => module !== "cover")
    //.filter(module => module === "flashcards")
}

export const ModuleSelection = (props?:Partial<Args> & {slot?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {slot, moduleKinds, } = props;

    return `
    <jig-edit-selection ${slot && `slot="${slot}"`}>
        ${mapToString(moduleKinds, module => {
            return Card({module, slot: "modules"});
        })}
    </jig-edit-selection>
    `;
}

ModuleSelection.args = DEFAULT_ARGS;
