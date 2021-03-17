import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/sections/sidebar/step1/widgets/single-list";
import "@elements/module/memory/edit/steps/sections/sidebar/step1/widgets/single-list-input";
import {mapToString, arrayCount} from "@utils/array";
const STR_CLEAR = "Clear list";

export default {
    title: "Module / Memory / Edit / Steps / Sections / Sidebar / Step1 / Widgets"
}

interface Args {
    nRows: number,
    placeholderCutoff: number
}

const DEFAULT_ARGS:Args = {
    nRows: 14,
    placeholderCutoff: 6 
}

export const SingleList = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nRows, placeholderCutoff} = props;

    return `
    <sidebar-widget-single-list slot="input-widget">
    ${mapToString(arrayCount(nRows), row => {

        const is_placeholder = row < placeholderCutoff;

        const value = is_placeholder ? "placeholder" : "value";
        const placeholder = is_placeholder ? "placeholder" : "";

        return`<sidebar-widget-single-list-input value="${value}" ${placeholder}></sidebar-widget-single-list-input>`
    })}
    </sidebar-widget-single-list>`
}

SingleList.args = DEFAULT_ARGS;
