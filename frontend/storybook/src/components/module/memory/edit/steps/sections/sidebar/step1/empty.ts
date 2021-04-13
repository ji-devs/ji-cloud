import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/module/memory/edit/steps/sections/sidebar/step1/empty";
import {MODE} from "@elements/module/memory/_common/types";

export default {
    title: "Module / Memory / Edit / Steps / Sections / Sidebar / Step1"
}

interface Args {
    mode: MODE
}

const DEFAULT_ARGS:Args = {
    mode: "duplicate"
}

export const  Empty = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `<step1-sidebar-empty slot="content" ${argsToAttrs(props)}></step1-sidebar-empty>`;
}

Empty.args = DEFAULT_ARGS;
