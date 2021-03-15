import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/step1/main/duplicate";
import "@elements/module/_common/main-empty";
import {mapToString, arrayIndex} from "@utils/array";
import {Pair} from "~/components/module/memory/edit/steps/_common/main/card-pair/pair";
export default {
    title: "Module / Memory / Edit / Steps / Step1 / Main"
}

interface Args {
    nCards: number
}

const DEFAULT_ARGS:Args = {
    nCards: 15
}

export const Duplicate = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nCards} = props;

    if(nCards <= 0) {
        return `<module-main-empty slot="main"></module-main-empty>`
    } else {
        return `
        <step1-main-duplicate slot="main">
        ${mapToString(arrayIndex(nCards), index => {
            return Pair({index})
            })}
        </step1-main-duplicate>`
    }

}

Duplicate.args = DEFAULT_ARGS;
