import {argsToAttrs} from "@utils/attributes";
import {DualList} from "./widgets/dual-list";

const STR_CLEAR = "Clear list";

export default {
    title: "Module / Memory / Edit / Steps / Sections / Sidebar / Step1"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Translate = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `
        <module-sidebar-body slot="content">
            ${DualList()}
        </module-sidebar-body>
            `
}

Translate.args = DEFAULT_ARGS;
