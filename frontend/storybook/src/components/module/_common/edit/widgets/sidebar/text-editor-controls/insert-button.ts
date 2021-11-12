import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/edit/widgets/text-editor-controls/insert-button";

export default {
    title: "Module / _COMMON / edit / Widgets / Sidebar / Text Editor Controls",
};

interface Args {
    disabled: boolean;
}

const DEFAULT_ARGS: Args = {
    disabled: false,
};

export const InsertButton = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <text-editor-controls-insert-button ${argsToAttrs(
            props
        )}></text-editor-controls-insert-button>
    `;
};

InsertButton.args = DEFAULT_ARGS;
