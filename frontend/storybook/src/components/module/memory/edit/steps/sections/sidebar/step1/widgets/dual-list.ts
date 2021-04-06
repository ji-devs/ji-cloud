import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/sections/sidebar/step1/widgets/dual-list";
import "@elements/module/memory/edit/steps/sections/sidebar/step1/widgets/dual-list-column";
import "@elements/module/memory/edit/steps/sections/sidebar/step1/widgets/dual-list-input";
import {mapToString, arrayCount} from "@utils/array";
const STR_CLEAR = "Clear list";

export default {
    title: "Module / Memory / Edit / Steps / Sections / Sidebar / Step1 / Widgets"
}

interface Args {
    nRows: number,
    placeholderCutoff: number,
    leftHeader: string, 
    rightHeader: string,
    nLines: number,
}

const DEFAULT_ARGS:Args = {
    nRows: 14,
    nLines: 3,
    placeholderCutoff: 6,
    leftHeader: "Left",
    rightHeader: "Right",

}

export const DualList = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nRows, nLines, placeholderCutoff, leftHeader, rightHeader} = props;

    return `
    <sidebar-widget-dual-list slot="input-widget">
        <sidebar-widget-dual-list-column side="left" header="${leftHeader}">
            ${mapToString(arrayCount(nRows), row => {

                const is_placeholder = row < placeholderCutoff;

                const value = is_placeholder ? "placeholder" : "value";
                const placeholder = is_placeholder ? "placeholder" : "";

                return`<sidebar-widget-dual-list-input value="${value}" nLines="${nLines}" ${placeholder}></sidebar-widget-dual-list-input>`
            })}
        </sidebar-widget-dual-list-column>
        <sidebar-widget-dual-list-column side="right" header="${rightHeader}">
            ${mapToString(arrayCount(nRows), row => {

                const is_placeholder = row < placeholderCutoff;

                const value = is_placeholder ? "placeholder" : "value";
                const placeholder = is_placeholder ? "placeholder" : "";

                return`<sidebar-widget-dual-list-input value="${value}" nLines="${nLines}" ${placeholder}></sidebar-widget-dual-list-input>`
            })}
        </sidebar-widget-dual-list-column>
    </sidebar-widget-dual-list>`
}

DualList.args = DEFAULT_ARGS;
