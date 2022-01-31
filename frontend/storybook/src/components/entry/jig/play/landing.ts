import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/jig/play/landing";

export default {
    title: "Entry / Jig / Play",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Landing = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <jig-play-landing ${argsToAttrs(props)}>
            <iframe slot="iframe" src="http://nothing/"></iframe>

            <jig-play-play-button slot="play-button"></jig-play-play-button>

            <jig-play-play-pause slot="play-pause-button" mode="play"></jig-play-play-pause>
            <jig-play-done-action slot="replay-background"></jig-play-done-action>
            <jig-play-background-music slot="background"></jig-play-background-music>
            <jig-play-points-indicator slot="indicators" value="210"></jig-play-points-indicator>
            <jig-play-timer-indicator slot="indicators" value="02:52"></jig-play-timer-indicator>

            <jig-play-move-button slot="back" kind="back"></jig-play-move-button>
            <jig-play-progress-bar slot="progress" percent="23"></jig-play-progress-bar>
            <jig-play-move-button slot="forward" kind="forward"></jig-play-move-button>
        </jig-play-landing>
    `;
};
Landing.args = DEFAULT_ARGS;
Landing.argTypes = {
    WW__WW: {
        control: {
            type: "inline-radio",
            options: [],
        },
    },
};
