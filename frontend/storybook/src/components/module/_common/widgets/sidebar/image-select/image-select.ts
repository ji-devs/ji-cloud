import "@elements/module/_common/widgets/image-select/image-select";
import "@elements/core/inputs/search";
import "@elements/core/inputs/checkbox";
import "@elements/core/buttons/text";
import { arrayCount, mapToString } from "@utils/array";

export default {
    title: "Module / _common / Widgets / Sidebar"
}

interface Args {
    imageCount: number,
}

const DEFAULT_ARGS:Args = {
    imageCount: 10,
}

export const ImageSelect = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <image-select label="Select background">
            <input-search placeholder="Search" slot="search-input"></input-search>
            <button-text slot="filters">Filters</button-text>
            <input-checkbox label="Show only background" slot="only-background-checkbox"></input-checkbox>
            <button-text slot="upload">Upload</button-text>
            ${mapToString(arrayCount(props.imageCount), () => {
                return `<img-ji slot="images" lib="mock" size="thumb" id="image.png"></img-ji>`
            })}
        </image-select>
    `;
}

ImageSelect.args = DEFAULT_ARGS;
