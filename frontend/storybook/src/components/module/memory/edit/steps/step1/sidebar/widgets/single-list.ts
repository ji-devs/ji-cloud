import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/step1/sidebar/widgets/single-list";
import {mapToString, arrayCount} from "@utils/array";
const STR_CLEAR = "Clear list";

export default {
    title: "Module / Memory / Edit / Steps / Step1 / Sidebar / Widgets"
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

SingleList.Args = DEFAULT_ARGS;
