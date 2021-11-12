import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-audio/jig-audio-play-pause";
import { Mode } from "@elements/entry/jig/edit/sidebar/settings/jig-audio/jig-audio-play-pause";

export default {
    title: "Entry / Jig / Edit / Sidebar / Settings / Jig Audio",
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
        <jig-audio-play-pause ${argsToAttrs(props)}></jig-audio-play-pause>
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
