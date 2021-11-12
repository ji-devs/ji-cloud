import { argsToAttrs } from "@utils/attributes";
import "@elements/core/menu/ellipses";

export default {
    title: "Core / Menu",
};

interface Args {
    hover: boolean;
    visible: boolean;
}

const DEFAULT_ARGS: Args = {
    hover: true,
    visible: true,
};

export const Ellipses = (props?: Args) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `<menu-ellipses ${argsToAttrs(props)}>
        <div slot="content">Menu Line Here</div>
        <div slot="menu-content">Menu Here</div>
    </menu-ellipses>`;
};

Ellipses.args = DEFAULT_ARGS;
