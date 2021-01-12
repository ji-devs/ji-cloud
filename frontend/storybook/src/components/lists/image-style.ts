import "@elements/lists/checkbox-list";
import "@elements/inputs/checkbox";
import {STYLE_OPTIONS} from "~/mock/meta";
import {mapToString} from "@utils/array";


export default {
  title: 'Lists/List',
}

interface Props {
    options: Array<String>
}

const DEFAULT_PROPS:Props = {
    options: STYLE_OPTIONS
}

export const ImageStyle = (props?:Props) => {
    const {options} = props || DEFAULT_PROPS;
    return `
        <checkbox-list title="Image style">
            ${mapToString(options, label => {
                return `<input-checkbox label="${label}"></input-checkbox>`
            })}

        </checkbox-list>
    `;
}

ImageStyle.args = DEFAULT_PROPS;
