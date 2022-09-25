import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/preview-button";

const STR_MY_JIGS = "My JIGs";
export default {
    title: "Entry / Jig / Edit / Sidebar",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const PreviewButton = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <jig-edit-sidebar-preview-button ${argsToAttrs(
            props
        )}></jig-edit-sidebar-preview-button>
    `;
};
PreviewButton.args = DEFAULT_ARGS;
