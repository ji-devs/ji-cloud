import "@elements/module/_common/widgets/image-select/image-select-style-option";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _common / Widgets / Sidebar / Image Select"
}

interface Args {
    selected: boolean,
    label: string,
}

const DEFAULT_ARGS:Args = {
    selected: true,
    label: "Label",
}

export const ImageSelectStyleOption = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <image-select-style-option ${argsToAttrs(props)}></image-select-style-option>
    `;
}

ImageSelectStyleOption.args = DEFAULT_ARGS;
