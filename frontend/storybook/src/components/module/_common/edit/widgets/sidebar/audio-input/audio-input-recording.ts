import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/edit/widgets/audio-input/audio-input-recording";

export default {
    title: "Module / _COMMON /  edit /Widgets / Sidebar / Audio Input"
}


interface Args {
}

const DEFAULT_ARGS: Args = {
}

export const AudioInputRecording = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <audio-input-recording ${argsToAttrs(props)}></audio-input-recording>
    `;
}

AudioInputRecording.args = DEFAULT_ARGS;

