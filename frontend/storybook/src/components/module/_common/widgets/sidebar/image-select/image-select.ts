import "@elements/module/_common/widgets/image-select/image-select";
import "@elements/module/_common/widgets/image-select/image-select-filters";
import "@elements/core/inputs/search";
import "@elements/core/inputs/checkbox";
import { arrayCount, mapToString } from "@utils/array";
import { argsToAttrs } from "@utils/attributes";
import { imageMode } from "@elements/module/_common/widgets/image-select/image-select";

export default {
    title: "Module / _common / Widgets / Sidebar / Image Select"
}

interface Args {
    label: string,
    filtersOpen: boolean,
    imageMode: imageMode,
    imageCount: number,
    hideOverlaySwitch: boolean,
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
    hideOverlaySwitch: true,
    seachBox: true,
    filters: true,
    showBackgroundCheckbox: true,
    uploadButton: true,
}

export const ImageSelect = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <image-select ${argsToAttrs(props)}>
            ${ props.hideOverlaySwitch && `<input-checkbox slot="hide-overlay" label="Hide overlay"></input-checkbox>` }
            ${ props.seachBox && `<input-search placeholder="Search" slot="search-input"></input-search>` }
            ${ props.filters && getFilters(props.filtersOpen) }
            ${ props.showBackgroundCheckbox && `<input-checkbox label="Show only background" slot="only-background-checkbox"></input-checkbox>` }
            ${ props.uploadButton && `<image-select-upload slot="upload" label="Upload"></image-select-upload>` }
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
        <image-select-filters slot="filters" ${ open ? "open" : "" }>
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
