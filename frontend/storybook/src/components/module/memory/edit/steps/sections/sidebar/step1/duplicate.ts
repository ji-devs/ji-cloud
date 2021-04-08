import {argsToAttrs} from "@utils/attributes";
import {SingleList} from "./widgets/single-list";

const STR_CLEAR = "Clear list";

export default {
    title: "Module / Memory / Edit / Steps / Sections / Sidebar / Step1"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Duplicate = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `
        <module-sidebar-body slot="content">
            ${SingleList()}
        </module-sidebar-body>
            `
}

Duplicate.args = DEFAULT_ARGS;
