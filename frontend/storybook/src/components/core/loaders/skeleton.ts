import { argsToAttrs } from "@utils/attributes";
import "@elements/core/loaders/skeleton";

export default {
    title: "Core / Loaders",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Skeleton = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <loader-skeleton style="height: 100px; width: 100px" ${argsToAttrs(props)}></loader-skeleton>
    `;
};

Skeleton.args = DEFAULT_ARGS;
