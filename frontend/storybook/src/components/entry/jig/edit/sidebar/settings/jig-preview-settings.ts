import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-preview-settings";

export default {
    title: "Entry / Jig / Edit / Sidebar / Settings",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const JigPreviewSettings = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <jig-preview-settings ${argsToAttrs(props)} slot="preview">
            <jig-preview-settings-direction></jig-preview-settings-direction>
            <label>
                <input-switch></input-switch>
                Display score
            </label>
            <label>
                <input-switch></input-switch>
                Assessment mode
            </label>
            <label>
                <input-switch></input-switch>
                Drag & Drop assist
            </label>
        </jig-preview-settings>
    `;
};
JigPreviewSettings.args = DEFAULT_ARGS;
