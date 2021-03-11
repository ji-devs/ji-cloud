import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/step1/main/duplicate";
import "@elements/module/_common/main-empty";

export default {
    title: "Module / Memory / Edit / Steps / Step1 / Main"
}

interface Args {
    empty: boolean
}

const DEFAULT_ARGS:Args = {
    empty: false 
}

export const Duplicate = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {empty} = props;

    if(empty) {
        return `<module-main-empty slot="main"></module-main-empty>`
    } else {
        return `
        <step1-main-duplicate slot="main">
        </step1-main-duplicate>`
    }

}

Duplicate.args = DEFAULT_ARGS;
