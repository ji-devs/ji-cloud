import { moduleKinds, ModuleKind } from "@elements/entry/jig/module-types";
import "@elements/module/_common/edit/post-preview/post-preview";
import "@elements/module/_common/edit/post-preview/post-preview-action";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _COMMON /  edit /Post Preview"
}

interface Args {
    module: ModuleKind;
}

const DEFAULT_ARGS:Args = {
    module: "memory",
}

export const PostPreview = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <post-preview ${argsToAttrs(props)}>
            <post-preview-action slot="action-1of3" kind="1of3"></post-preview-action>
            <post-preview-action slot="action-matching" kind="matching"></post-preview-action>
            <post-preview-action slot="action-flashcards" kind="flashcards"></post-preview-action>
            <post-preview-action slot="action-print" kind="print"></post-preview-action>
            <post-preview-action slot="action-continue" kind="continue"></post-preview-action>
        </post-preview>
    `;
}

PostPreview.args = DEFAULT_ARGS;
PostPreview.argTypes = {
    module: {
        control: {
            type: 'inline-radio',
            options: ["cover", "flashcards", "matching", "memory", "poster", "tapping-board", "tracing", "video", "visual-quiz"],
        }
    }
}