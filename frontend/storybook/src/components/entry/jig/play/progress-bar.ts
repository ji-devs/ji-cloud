import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/jig/play/progress-bar";

export default {
    title: "Entry / Jig / Play",
};

interface Args {
    percent: number;
}

const DEFAULT_ARGS: Args = {
    percent: 34,
};

export const ProgressBar = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <jig-play-progress-bar ${argsToAttrs(props)}></jig-play-progress-bar>
    `;
};
ProgressBar.args = DEFAULT_ARGS;
