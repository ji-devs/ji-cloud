import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/edit/widgets/audio-input/audio-input-icon";
import { IconKind } from "@elements/module/_common/edit/widgets/audio-input/audio-input-icon";

export default {
    title: "Module / _COMMON /  edit /Widgets / Sidebar / Audio Input",
};

interface Args {
    kind: IconKind;
}

const DEFAULT_ARGS: Args = {
    kind: "record",
};

export const AudioInputIcon = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <audio-input-icon ${argsToAttrs(props)}></audio-input-icon>
    `;
};

AudioInputIcon.args = DEFAULT_ARGS;

AudioInputIcon.argTypes = {
    kind: {
        control: {
            type: "inline-radio",
            options: ["record", "success", "upload"],
        },
    },
};
