import "@elements/lists/checkbox-list";
import "@elements/inputs/checkbox";
import {STREAM_OPTIONS} from "~/mock/meta";
import {mapToString} from "@utils/array";


export default {
  title: 'Lists/List',
}

interface Props {
    options: Array<String>
}

const DEFAULT_PROPS:Props = {
    options: STREAM_OPTIONS
}

export const Stream = (props?:Props) => {
    const {options} = props || DEFAULT_PROPS;
    return `
        <checkbox-list title="Content from which streams of Judaism do you want to see?">
            ${mapToString(options, label => {
                return `<input-checkbox label="${label}"></input-checkbox>`
            })}

        </checkbox-list>
    `;
}

Stream.args = DEFAULT_PROPS;

