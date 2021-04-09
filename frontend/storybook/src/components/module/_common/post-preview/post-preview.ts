import "@elements/module/_common/post-preview/post-preview";
import "@elements/module/_common/post-preview/post-preview-action";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _common / Post Preview"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const PostPreview = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <post-preview ${argsToAttrs(props)}>
            <p slot="message">Your memory game is ready!</p>
            <p slot="message">It’s now part of your JIG.</p>
            <post-preview-action slot="action-_somthing_" kind="_somthing_"></post-preview-action>
            <post-preview-action slot="action-matching" kind="matching"></post-preview-action>
            <post-preview-action slot="action-flashcards" kind="flashcards"></post-preview-action>
            <post-preview-action slot="action-print" kind="print"></post-preview-action>
            <post-preview-action slot="action-continue" kind="continue"></post-preview-action>
        </post-preview>
    `;
}

PostPreview.args = DEFAULT_ARGS;
