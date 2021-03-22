import "@elements/module/_common/widgets/image-select/image-select-filters";
import "@elements/module/_common/widgets/image-select/image-select-style-option";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _common / Widgets / Sidebar / Image Select"
}

interface Args {
    open: boolean,
}

const DEFAULT_ARGS:Args = {
    open: true,
}

export const ImageSelectFilters = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <image-select-filters
            slot="filters"
            ${argsToAttrs(props)}
        >
            <label slot="source-options">
                <input type="radio" name="type" value="web" checked>
                Stickers
            </label>
            <label slot="source-options">
                <input type="radio" name="type" value="stikers">
                Web
            </label>
            <image-select-style-option slot="style-options" label="All"></image-select-style-option>
            <image-select-style-option slot="style-options" label="Animated" selected></image-select-style-option>
            <image-select-style-option slot="style-options" label="Clipart"></image-select-style-option>
            <image-select-style-option slot="style-options" label="Photo"></image-select-style-option>
            <image-select-style-option slot="style-options" label="Transparent"></image-select-style-option>
        </image-select-filters>
    `;
}

ImageSelectFilters.args = DEFAULT_ARGS;
