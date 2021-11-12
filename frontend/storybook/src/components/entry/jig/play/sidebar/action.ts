import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/jig/play/sidebar/action";
import { Kind } from "@elements/entry/jig/play/sidebar/action";

export default {
    title: "Entry / Jig / Play / Sidebar",
};

interface Args {
    kind: Kind;
    active: boolean;
}

const DEFAULT_ARGS: Args = {
    kind: "info",
    active: false,
};

export const Action = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <jig-play-sidebar-action ${argsToAttrs(
            props
        )}></jig-play-sidebar-action>
    `;
};
Action.args = DEFAULT_ARGS;
Action.argTypes = {
    kind: {
        control: {
            type: "inline-radio",
            options: ["like", "share", "info"],
        },
    },
};
