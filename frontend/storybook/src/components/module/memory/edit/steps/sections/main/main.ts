import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/main-empty";
import "@elements/module/memory/edit/steps/sections/main/main";
import {mapToString, arrayIndex} from "@utils/array";
import {Pair} from "~/components/module/memory/edit/steps/sections/main/card-pair/pair";
import {Args as CardArgs} from "~/components/module/memory/edit/steps/sections/main/card-pair/card";
export default {
    title: "Module / Memory / Edit / Steps / Sections"
}

interface Args {
    nCards: number
    leftArgs?: CardArgs
    rightArgs?: CardArgs
}

const DEFAULT_ARGS:Args = {
    nCards: 6
}

export const Main = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nCards, leftArgs, rightArgs} = props;

    if(nCards <= 0) {
        return `<module-main-empty slot="main"></module-main-empty>`
    } else {
        return `
        <main-cards slot="main">
        ${mapToString(arrayIndex(nCards), index => {
            return Pair({index, leftArgs, rightArgs})
            })}
        </main-cards>`
    }

}

Main.args = DEFAULT_ARGS;
