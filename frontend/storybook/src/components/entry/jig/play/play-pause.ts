import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/jig/play/play-pause";
import { Mode } from "@elements/entry/jig/play/play-pause";

export default {
    title: "Entry / Jig / Play",
};

interface Args {
    mode: Mode;
}

const DEFAULT_ARGS: Args = {
    mode: "play",
};

export const PlayPause = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <jig-play-play-pause ${argsToAttrs(props)}></jig-play-play-pause>
    `;
};
PlayPause.args = DEFAULT_ARGS;
PlayPause.argTypes = {
    mode: {
        control: {
            type: "inline-radio",
            options: ["play", "pause"],
        },
    },
};
