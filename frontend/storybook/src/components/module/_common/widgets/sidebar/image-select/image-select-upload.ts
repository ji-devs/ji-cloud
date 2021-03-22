import "@elements/module/_common/widgets/image-select/image-select-upload";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _common / Widgets / Sidebar / Image Select"
}

interface Args {
    label: string,
}

const DEFAULT_ARGS:Args = {
    label: "Label",
}

export const ImageSelectUpload = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <image-select-upload ${argsToAttrs(props)}></image-select-upload>
    `;
}

ImageSelectUpload.args = DEFAULT_ARGS;
