import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/sections/sidebar/step1/duplicate";
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
    <step1-sidebar-duplicate slot="content">
        <button-text slot="clear">${STR_CLEAR}</button-text>
        <button-sidebar slot="input-buttons" mode="keyboard"></button-sidebar>
        <button-sidebar slot="input-buttons" mode="dicta"></button-sidebar>
        <button-sidebar slot="input-buttons" mode="sefaria"></button-sidebar>
        ${SingleList()}
    </step1-sidebar-duplicate>`
}

Duplicate.args = DEFAULT_ARGS;
