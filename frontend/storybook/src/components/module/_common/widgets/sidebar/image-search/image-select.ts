import "@elements/module/_common/widgets/image-search/image-select";
import "@elements/module/_common/widgets/image-search/image-search-filters";
import "@elements/core/inputs/search";
import "@elements/core/inputs/checkbox";
import "@elements/core/inputs/switch";
import { arrayCount, mapToString } from "@utils/array";
import { argsToAttrs } from "@utils/attributes";
import { imageMode } from "@elements/module/_common/widgets/image-search/image-select";

export default {
    title: "Module / _common / Widgets / Sidebar / Image Search"
}

interface Args {
    label: string,
    filtersOpen: boolean,
    imageMode: imageMode,
    imageCount: number,
    seachBox: boolean,
    filters: boolean,
    showBackgroundCheckbox: boolean,
    uploadButton: boolean,
}

const DEFAULT_ARGS:Args = {
    label: "Select background",
    filtersOpen: false,
    imageMode: 'image',
    imageCount: 10,
    seachBox: true,
    filters: true,
    showBackgroundCheckbox: true,
    uploadButton: true,
}

export const ImageSelect = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <image-select ${argsToAttrs(props)}>
            ${ props.seachBox && `<input-search placeholder="Search" slot="search-input"></input-search>` }
            ${ props.filters && getFilters(props.filtersOpen) }
            ${ props.showBackgroundCheckbox && `<input-checkbox label="Show only background" slot="only-background-checkbox"></input-checkbox>` }
            ${ props.uploadButton && `<image-search-upload slot="upload" label="Upload"></image-search-upload>` }
            ${mapToString(arrayCount(props.imageCount), () => {
                return `<img-ji slot="images" lib="mock" size="thumb" id="image.png"></img-ji>`
            })}
        </image-select>
    `;
}

ImageSelect.args = DEFAULT_ARGS;
ImageSelect.argTypes = {
    imageMode: {
        control: {
            type: 'inline-radio',
            options: ['image', 'background']
        }
    }
}

function getFilters(open: boolean): string {
    return `
        <image-search-filters slot="filters" ${ open ? "open" : "" }>
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
