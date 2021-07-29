import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/edit/widgets/audio-input/audio-input";
import { mode } from "@elements/module/_common/edit/widgets/audio-input/audio-input";
import "@elements/module/_common/edit/widgets/audio-input/audio-input-action";
import "@elements/module/_common/edit/widgets/audio-input/audio-input-icon";

export default {
    title: "Module / _COMMON /  edit /Widgets / Sidebar / Audio Input"
}


interface Args {
    mode: mode,
}

const DEFAULT_ARGS: Args = {
    mode: 'default',
}

export const AudioInput = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <audio-input ${argsToAttrs(props)} style="width: 492px">
            <label slot="options">
                <input type="radio" name="type" value="record" checked>
                Record
            </label>
            <label slot="options">
                <input type="radio" name="type" value="upload">
                Upload a file
            </label>
            <div slot="main-content">
                <audio-input-icon kind="success"></audio-input-icon>
            </div>
            <button-rect kind="text" slot="delete" color="blue">Delete</button-rect>
            <audio-input-action slot="main-action" kind="play"></audio-input-action>
        </audio-input>
    `;
}

AudioInput.args = DEFAULT_ARGS;

AudioInput.argTypes = {
    mode: {
        control: {
            type: 'inline-radio',
            options: ['default', 'active', 'success', 'done']
        }
    }
}
