import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_groups/cards/edit/main/main";
import "@elements/module/_groups/cards/_common/main-empty";
import { mapToString, arrayIndex } from "@utils/array";
import { Pair } from "./card-pair/pair";
import { Args as CardArgs } from "./card-pair/card";
export default {
    title: "Module / _GROUPS / Cards / edit / Main",
};

interface Args {
    nCards: number;
    leftArgs?: CardArgs;
    rightArgs?: CardArgs;
}

const DEFAULT_ARGS: Args = {
    nCards: 6,
};

export const Container = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    const { nCards, leftArgs, rightArgs } = props;

    if (nCards <= 0) {
        return `<main-empty slot="main"></main-empty>`;
    } else {
        return `
        <main-cards slot="main">
        ${mapToString(arrayIndex(nCards), (index) => {
            return Pair({ index, leftArgs, rightArgs });
        })}
        </main-cards>`;
    }
};

Container.args = DEFAULT_ARGS;
