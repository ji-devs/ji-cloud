import { moduleKinds, ModuleKind } from "@elements/module/_common/types";
import "@elements/entry/jig/edit/post-publish/post-publish";
import "@elements/entry/jig/edit/post-publish/post-publish-action";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Entry / Jig / Edit / Post publish",
};

interface Args {
    module: ModuleKind;
}

const DEFAULT_ARGS: Args = {
    module: "memory",
};

export const PostPublish = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <post-publish ${argsToAttrs(props)}>
            <post-publish-action slot="actions" kind="share"></post-publish-action>
            <post-publish-action slot="actions" kind="new-jig"></post-publish-action>
            <post-publish-action slot="actions" kind="play-jig"></post-publish-action>
        </post-publish>
    `;
};

PostPublish.args = DEFAULT_ARGS;
