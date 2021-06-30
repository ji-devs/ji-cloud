import "@elements/module/_common/edit/widgets/audio-input/audio-input";
import { mode as AudioInputMode } from "@elements/module/_common/edit/widgets/audio-input/audio-input";
import "@elements/module/_common/edit/widgets/audio-input/audio-input-recording";
import "@elements/module/_common/edit/widgets/audio-input/audio-input-action";
import "@elements/module/_common/edit/widgets/audio-input/audio-input-delete";
import "@elements/module/_common/edit/widgets/audio-input/audio-input-icon";
import "@elements/core/progress-bar/progress-bar";
import "@elements/core/inputs/primitives/file";
import "@elements/core/buttons/text";

export default {
    title: "Module / _COMMON /  edit /Widgets / Sidebar / Audio Input"
}

type PreviewMode = 'Record' | 'Recording' | 'Recorded' | 'Playing Recorded File' | 'Upload / Browse' | 'Uploading' | 'File Uploaded' | 'Playing Uploaded File';


interface Args {
    mode: PreviewMode,
}

const DEFAULT_ARGS: Args = {
    mode: 'Record',
}

export const AudioInputFull = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;
    const mode = props.mode;
    const checked = getCheckboxes(mode);

    return `
        <audio-input mode="${getAudioInputMode(mode)}" style="width: 492px">
            <label slot="options">
                <input type="radio" name="type" value="record" ${ checked === 'record' && 'checked' }>
                Record
            </label>
            <label slot="options">
                <input type="radio" name="type" value="upload" ${ checked === 'file' && 'checked' }>
                Upload a file
            </label>
            ${ getMainContent(mode) }
            ${ showDelete(mode) && `<audio-input-delete slot="delete"></audio-input-delete>` }
            ${ getAction(mode) }
        </audio-input>
    `;
}

AudioInputFull.args = DEFAULT_ARGS;

AudioInputFull.argTypes = {
    mode: {
        control: {
            type: 'inline-radio',
            options: ['Record', 'Recording', 'Recorded', 'Playing Recorded File', 'Upload / Browse', 'Uploading', 'File Uploaded', 'Playing Uploaded File'],
        }
    }
}

function getCheckboxes(previewMode: PreviewMode): 'record' | 'file' {
    if(
        previewMode === "Record"
        ||
        previewMode === "Recording"
        ||
        previewMode === "Recorded"
        ||
        previewMode === "Playing Recorded File"
    ) {
        return 'record';
    } else {
        return 'file';
    }
}

function getMainContent(previewMode: PreviewMode): string {
    switch (previewMode) {
        case 'Record':
            return `<audio-input-icon slot="main-content" kind="record"></audio-input-icon>`;
        case 'Recording':
            return `<audio-input-recording slot="main-content"></audio-input-recording>`;
        case 'Recorded':
            return `<audio-input-icon slot="main-content" kind="success"></audio-input-icon>`;
        case 'Playing Recorded File':
            return `<progress-bar slot="main-content" progress="23" color="green"></progress-bar>`;
        case 'Upload / Browse':
            return `<input-file slot="main-content"><audio-input-icon kind="upload"></audio-input-icon></input-file>`;
        case 'Uploading':
            return `<progress-bar slot="main-content" progress="0" color="blue"></progress-bar>`;
        case 'File Uploaded':
            return `<audio-input-icon slot="main-content" kind="success"></audio-input-icon>`;
        case 'Playing Uploaded File':
            return `<progress-bar slot="main-content" progress="0" color="green"></progress-bar>`;
    }
}

function getAudioInputMode(previewMode: PreviewMode): AudioInputMode {
    switch (previewMode) {
        case 'Record':
        case 'Upload / Browse':
            return "default";
        case 'Recording':
        case 'Uploading':
            return "active";
        case 'Recorded':
        case 'File Uploaded':
            return "success";
        case 'Playing Recorded File':
        case 'Playing Uploaded File':
            return "done";
    }
}


function getAction(previewMode: PreviewMode): string {
    switch (previewMode) {
        case 'Record':
            return `<audio-input-action slot="main-action" kind="record"></audio-input-action>`;
        case 'Recording':
            return `<audio-input-action slot="main-action" kind="stop"></audio-input-action>`;
        case 'Recorded':
            return `<audio-input-action slot="main-action" kind="play"></audio-input-action>`;
        case 'Playing Recorded File':
            return `<audio-input-action slot="main-action" kind="stop"></audio-input-action>`;
        case 'Upload / Browse':
            return `<audio-input-action slot="main-action" kind="add-sound"></audio-input-action>`;
        case 'Uploading':
            return `<button-text slot="main-action">Cancel</button-text>`;
        case 'File Uploaded':
            return `<audio-input-action slot="main-action" kind="play"></audio-input-action>`;
        case 'Playing Uploaded File':
            return `<audio-input-action slot="main-action" kind="stop"></audio-input-action>`;
    }
}

function showDelete(previewMode: PreviewMode): boolean {
    return previewMode === "Recorded" || previewMode === "Playing Recorded File" || previewMode === "File Uploaded" || previewMode === "Playing Uploaded File";
}
