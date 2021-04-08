import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/widgets/audio-input/audio-input-delete";

export default {
    title: "Module / _common / Widgets / Sidebar / Audio Input"
}


interface Args {
}

const DEFAULT_ARGS: Args = {
}

export const AudioInputDelete = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <audio-input-delete ${argsToAttrs(props)}></audio-input-delete>
    `;
}

AudioInputDelete.args = DEFAULT_ARGS;
