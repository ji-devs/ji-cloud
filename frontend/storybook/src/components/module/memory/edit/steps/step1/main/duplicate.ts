import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/step1/main/duplicate";
export default {
    title: "Module / Memory / Edit / Steps / Step1 / Main"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Duplicate = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `
    <step1-main-duplicate slot="main">
    </step1-main-duplicate>`

}

Duplicate.Args = DEFAULT_ARGS;
