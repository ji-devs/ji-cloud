import "@elements/module/_common/widgets/image-search/image-search-upload";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _common / Widgets / Sidebar / Image Search"
}

interface Args {
    label: string,
}

const DEFAULT_ARGS:Args = {
    label: "Label",
}

export const ImageSearchUpload = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <image-search-upload ${argsToAttrs(props)}></image-search-upload>
    `;
}

ImageSearchUpload.args = DEFAULT_ARGS;
