import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/edit/widgets/lists/single/single-list";
import "@elements/module/_common/edit/widgets/lists/single/single-list-input";
import {mapToString, arrayCount} from "@utils/array";
const STR_CLEAR = "Clear list";
const STR_DONE = "Done";

export default {
    title: "Module / _COMMON /  edit / Widgets / Sidebar / List "
}

interface Args {
    nRows: number,
    placeholderCutoff: number
}

const DEFAULT_ARGS:Args = {
    nRows: 14,
    placeholderCutoff: 6 
}

export const Single = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nRows, placeholderCutoff} = props;

    return `
    <sidebar-widget-single-list>
        <button-rect kind="text" slot="clear">${STR_CLEAR}</button-rect>
        <button-sidebar slot="input-buttons" mode="keyboard"></button-sidebar>
        <button-sidebar slot="input-buttons" mode="dicta"></button-sidebar>
        <button-sidebar slot="input-buttons" mode="sefaria"></button-sidebar>
        <button-rect disabled size="small" iconAfter="done" slot="done-btn">${STR_DONE}</button-rect>
    ${mapToString(arrayCount(nRows), row => {

        const is_placeholder = row < placeholderCutoff;

        const value = is_placeholder ? "placeholder" : "value";
        const placeholder = is_placeholder ? "placeholder" : "";

        return`<sidebar-widget-single-list-input value="${value}" ${placeholder}></sidebar-widget-single-list-input>`
    })}
    </sidebar-widget-single-list>`
}

Single.args = DEFAULT_ARGS;