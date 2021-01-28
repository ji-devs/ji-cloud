import "@elements/core/lists/list-vertical";
import "@elements/core/inputs/checkbox";
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

const STR_STYLE = "Image style"

export const ImageStyle = (props?:Props) => {
    const {options} = props || DEFAULT_PROPS;
    return `
        <list-vertical label="${STR_STYLE}">
            ${mapToString(options, label => {
                return `<input-checkbox label="${label}"></input-checkbox>`
            })}

        </list-vertical>
    `;
}

ImageStyle.args = DEFAULT_PROPS;
