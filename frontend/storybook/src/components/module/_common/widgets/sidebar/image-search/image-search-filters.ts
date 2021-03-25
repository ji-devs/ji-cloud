import "@elements/module/_common/widgets/image-search/image-search-filters";
import "@elements/module/_common/widgets/image-search/image-search-style-option";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _common / Widgets / Sidebar / Image Search"
}

interface Args {
    open: boolean,
}

const DEFAULT_ARGS:Args = {
    open: true,
}

export const ImageSearchFilters = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <image-search-filters
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
            <image-search-style-option slot="style-options" label="All"></image-search-style-option>
            <image-search-style-option slot="style-options" label="Animated" selected></image-search-style-option>
            <image-search-style-option slot="style-options" label="Clipart"></image-search-style-option>
            <image-search-style-option slot="style-options" label="Photo"></image-search-style-option>
            <image-search-style-option slot="style-options" label="Transparent"></image-search-style-option>
        </image-search-filters>
    `;
}

ImageSearchFilters.args = DEFAULT_ARGS;
