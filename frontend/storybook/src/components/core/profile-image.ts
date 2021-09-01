import {argsToAttrs, argToAttr} from "@utils/attributes";
import "@elements/core/profile-image";


export default {
    title: "Core / Profile image"
}

interface Args {
    imageId: string
}

const DEFAULT_ARGS:Args = {
    imageId: ""
}

export const ProfileImage = (props?:Args) => {
    return `<profile-image ${argsToAttrs(props)}></profile-image>`;
}

ProfileImage.args = DEFAULT_ARGS;
