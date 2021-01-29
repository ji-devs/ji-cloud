import "@elements/core/lists/list-vertical";
import "@elements/core/inputs/checkbox";
import {AFFILIATION_OPTIONS} from "~/mock/meta";
import {mapToString} from "@utils/array";


export default {
  title: 'Entry / Admin / Images / Meta / Lists',
}

interface Props {
    options: Array<String>
}

const DEFAULT_PROPS:Props = {
    options: AFFILIATION_OPTIONS
}
const STR_STREAM = "Suitable for jewish stream?";

export const Stream = (props?:Props) => {
    const {options} = props || DEFAULT_PROPS;
    return `
        <list-vertical label="${STR_STREAM}">
            ${mapToString(options, label => {
                return `<input-checkbox label="${label}"></input-checkbox>`
            })}

        </list-vertical>
    `;
}

Stream.args = DEFAULT_PROPS;

