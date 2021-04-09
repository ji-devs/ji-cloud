import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/widgets/text-editor-controls/text-editor-controls";
import "@elements/module/_common/widgets/text-editor-controls/text-editor-control";
import "@elements/core/buttons/button-collection/button-collection";
import "@elements/core/buttons/sidebar";
import "@elements/core/inputs/dropdowns/dropdown-select";
import "@elements/core/inputs/inc-dec";
import "@elements/core/lists/li-check";

export default {
    title: "Module / _common / Widgets / Sidebar"
}


interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const TextEditorControls = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <text-editor-controls ${argsToAttrs(props)} style="width: 492px">
            <dropdown-select slot="font" label="Font" value="Arial">
                <li-check selected>Options</li-check>
            </dropdown-select>
            <button-collection slot="type">
                <text-editor-control type="h1"></text-editor-control>
                <text-editor-control type="h2"></text-editor-control>
                <text-editor-control type="p1"></text-editor-control>
                <text-editor-control type="p2"></text-editor-control>
            </button-collection>
            <dropdown-select slot="weight" label="Weight" value="Normal">
                <li-check selected>Options</li-check>
            </dropdown-select>
            <input-inc-dec slot="font-size"></input-inc-dec>
            <button-collection slot="style">
                <text-editor-control type="bold"></text-editor-control>
                <text-editor-control type="italic"></text-editor-control>
                <text-editor-control type="underline"></text-editor-control>
            </button-collection>
            <button-collection slot="color">
                <text-editor-control type="color"></text-editor-control>
                <text-editor-control type="marker-color"></text-editor-control>
            </button-collection>
            <button-collection slot="justify">
                <text-editor-control type="align-left"></text-editor-control>
                <text-editor-control type="align-center"></text-editor-control>
                <text-editor-control type="align-right"></text-editor-control>
                <text-editor-control type="dir-ltr"></text-editor-control>
                <text-editor-control type="dir-rtl"></text-editor-control>
            </button-collection>
            <button-sidebar slot="hewbrew-keyboard" mode="keyboard"></button-sidebar>
            <button-sidebar slot="dicta" mode="dicta"></button-sidebar>
            <button-sidebar slot="sefaria" mode="sefaria"></button-sidebar>
        </text-editor-controls>
    `;
}

TextEditorControls.args = DEFAULT_ARGS;
