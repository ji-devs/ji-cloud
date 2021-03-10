import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/step1/sidebar/duplicate";
import {SingleList} from "./widgets/single-list";

const STR_CLEAR = "Clear list";
const STR_DONE = "Done";

export default {
    title: "Module / Memory / Edit / Steps / Step1 / Sidebar"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Duplicate = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `
    <step1-sidebar-duplicate>
        <button-text slot="clear">${STR_CLEAR}</button-text>
        <button-sidebar slot="input-buttons" mode="keyboard"></button-sidebar>
        <button-sidebar slot="input-buttons" mode="dicta"></button-sidebar>
        <button-sidebar slot="input-buttons" mode="sefaria"></button-sidebar>
        ${SingleList()}
        <button-rect color="grey" size="small" iconAfter="done" slot="btn-done">${STR_DONE}</button-rect>
    </step1-sidebar-duplicate>`
}

Duplicate.Args = DEFAULT_ARGS;
