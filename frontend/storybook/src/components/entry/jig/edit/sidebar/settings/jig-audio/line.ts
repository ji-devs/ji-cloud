import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-audio/jig-audio-line";
import "@elements/entry/jig/edit/sidebar/settings/jig-audio/jig-audio-play-pause";

export default {
    title: "Entry / Jig / Edit / Sidebar/ Settings/ Jig Audio"
}

interface Args {
    selected: boolean,
    playing: boolean,
    new: boolean,
    label: string;
}

const DEFAULT_ARGS:Args = {
    selected: false,
    playing: false,
    new: false,
    label: "Hanerot Halalu",
}

export const Line = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-audio-line ${argsToAttrs(props)}>
            <jig-audio-play-pause mode="${props.playing ? "pause" : "play" }" slot="play-pause"></jig-audio-play-pause>
        </jig-audio-line>
    `;
}
Line.args = DEFAULT_ARGS;
