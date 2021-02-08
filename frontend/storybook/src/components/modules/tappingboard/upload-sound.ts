import {argsToAttrs} from "@utils/attributes";
import "@elements/modules/tappingboard/upload-sound";

export default {
    title: "Modules/Tappingboard/Widgets"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const UploadSound = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<upload-sound ${argsToAttrs(props)}></upload-sound>`;
}

UploadSound.args = DEFAULT_ARGS;