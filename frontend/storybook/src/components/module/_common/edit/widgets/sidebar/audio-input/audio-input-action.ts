import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/edit/widgets/audio-input/audio-input-action";
import { Kind } from "@elements/module/_common/edit/widgets/audio-input/audio-input-action";

export default {
    title: "Module / _COMMON /  edit /Widgets / Sidebar / Audio Input"
}


interface Args {
    kind: Kind,
    disabled: boolean,
}

const DEFAULT_ARGS: Args = {
    kind: "record",
    disabled: false,
}

export const AudioInputAction = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <audio-input-action ${argsToAttrs(props)}></audio-input-action>
    `;
}

AudioInputAction.args = DEFAULT_ARGS;

AudioInputAction.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ["record", "confirm", "preview", "play", "stop", "add-sound"]
        }
    }
}
