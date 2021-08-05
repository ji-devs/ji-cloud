import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/edit/widgets/text-editor-controls/text-editor-controls";

export default {
    title: "Module / _COMMON / edit / Widgets / Sidebar / Text Editor Controls"
}


interface Args {
    controlsDisabled: boolean,
}

const DEFAULT_ARGS:Args = {
    controlsDisabled: false,
}

export const TextEditorControls = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <text-editor-controls ${argsToAttrs(props)} style="width: 492px">
            <text-editor-controls-insert-button slot="insert-button"></text-editor-controls-insert-button>
            <input-select slot="font" label="Font" value="Arial">
                <input-select-option selected>Options</input-select-option>
            </input-select>
            <text-editor-controls-button slot="h1" kind="h1"></text-editor-controls-button>
            <text-editor-controls-button slot="h2" kind="h2"></text-editor-controls-button>
            <text-editor-controls-button slot="p1" kind="p1"></text-editor-controls-button>
            <text-editor-controls-button slot="p2" kind="p2"></text-editor-controls-button>
            <input-select slot="weight" label="Weight" value="Normal">
                <input-select-option selected>Options</input-select-option>
            </input-select>
            <text-editor-controls-input-number slot="font-size"></text-editor-controls-input-number>
            <text-editor-controls-button slot="bold" kind="bold"></text-editor-controls-button>
            <text-editor-controls-button slot="italic" kind="italic"></text-editor-controls-button>
            <text-editor-controls-button slot="underline" kind="underline"></text-editor-controls-button>
            <text-editor-controls-button slot="color" kind="color"></text-editor-controls-button>
            <text-editor-controls-button slot="highlight-color" kind="highlight-color"></text-editor-controls-button>
            <text-editor-controls-button slot="align-left" kind="align-left"></text-editor-controls-button>
            <text-editor-controls-button slot="align-center" kind="align-center"></text-editor-controls-button>
            <text-editor-controls-button slot="align-right" kind="align-right"></text-editor-controls-button>
            <text-editor-controls-button slot="indent" kind="indent"></text-editor-controls-button>
            <text-editor-controls-button slot="outdent" kind="outdent"></text-editor-controls-button>
        </text-editor-controls>
    `;
}

TextEditorControls.args = DEFAULT_ARGS;
