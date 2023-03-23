import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/asset/edit/selection/jig-selection";
import { mapToString, arrayIndex } from "@utils/array";
import { Card } from "./card";
import { ModuleKind, moduleKinds } from "@elements/module/_common/types";
export default {
    title: "Entry / Jig / Edit / Selection",
};

interface Args {
    moduleKinds: Array<ModuleKind>;
}

const DEFAULT_ARGS: Args = {
    moduleKinds: moduleKinds.filter((module) => module !== "cover"),
    //.filter(module => module === "flashcards")
};

export const ModuleSelection = (props?: Partial<Args> & { slot?: string }) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    const { slot, moduleKinds } = props;

    return `
    <asset-edit-jig-selection ${slot && `slot="${slot}"`}>
        ${mapToString(moduleKinds, (module) => {
            return Card({ module, slot: "modules" });
        })}
    </asset-edit-jig-selection>
    `;
};

ModuleSelection.args = DEFAULT_ARGS;
