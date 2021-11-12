import "@elements/core/icon/fa-icon";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Core / Icon",
};

interface Args {
    icon: string;
}

const DEFAULT_ARGS: Args = {
    icon: "fa-solid fa-stars",
};

export const FaIcon = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `<fa-icon ${argsToAttrs(props)}></fa-icon>`;
};

FaIcon.args = DEFAULT_ARGS;
