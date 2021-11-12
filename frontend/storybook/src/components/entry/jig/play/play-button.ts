import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/jig/play/play-button";

export default {
    title: "Entry / Jig / Play",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const PlayButton = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <jig-play-play-button ${argsToAttrs(props)}></jig-play-play-button>
    `;
};
PlayButton.args = DEFAULT_ARGS;
