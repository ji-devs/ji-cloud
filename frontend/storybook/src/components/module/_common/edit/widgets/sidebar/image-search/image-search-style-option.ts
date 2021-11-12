import "@elements/module/_common/edit/widgets/image-search/image-search-style-option";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _COMMON /  edit /Widgets / Sidebar / Image Search",
};

interface Args {
    selected: boolean;
    label: string;
}

const DEFAULT_ARGS: Args = {
    selected: true,
    label: "Label",
};

export const ImageSearchStyleOption = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <image-search-style-option ${argsToAttrs(
            props
        )}></image-search-style-option>
    `;
};

ImageSearchStyleOption.args = DEFAULT_ARGS;
