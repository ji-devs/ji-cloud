import "@elements/module/_common/edit/post-preview/post-preview-action";
import { argsToAttrs } from "@utils/attributes";
import { Kind } from "@elements/module/_common/edit/post-publish/post-publish-action";

export default {
    title: "Entry / Jig / Edit / Post publish",
};

interface Args {
    kind: Kind;
}

const DEFAULT_ARGS: Args = {
    kind: "flashcards",
};

export const action = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <post-publish-action ${argsToAttrs(props)}></post-publish-action>
    `;
};

action.args = DEFAULT_ARGS;
action.argTypes = {
    kind: {
        control: {
            type: "inline-radio",
            options: ["share", "new-jig", "play-jig"],
        },
    },
};
