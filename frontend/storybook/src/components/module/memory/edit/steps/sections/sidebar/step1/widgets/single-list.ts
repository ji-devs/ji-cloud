import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/sections/sidebar/step1/widgets/single-list";
import {mapToString, arrayCount} from "@utils/array";
const STR_CLEAR = "Clear list";

export default {
    title: "Module / Memory / Edit / Steps / Sections / Sidebar / Step1 / Widgets"
}

interface Args {
    nRows: number,
    placeholder: boolean
}

const DEFAULT_ARGS:Args = {
    nRows: 14,
    placeholder: true 
}

export const SingleList = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nRows, placeholder} = props;

    return `
    <sidebar-widget-single-list slot="input-widget">
    ${mapToString(arrayCount(nRows), row => {

        const value = row < 6 
            ? placeholder ? "placeholder='placeholder'" : "value='value'"
            : "";

        return`<sidebar-widget-single-list-input ${value}></sidebar-widget-single-list-input>`
    })}
    </sidebar-widget-single-list>`
}

SingleList.args = DEFAULT_ARGS;
