import "@elements/lists/checkbox-list";
import "@elements/inputs/checkbox";
import {AGETWO_OPTIONS} from "~/mock/meta";
import {mapToString} from "@utils/array";


export default {
  title: 'Lists/List',
}
interface Props {
    options: Array<String>
}

const DEFAULT_PROPS:Props = {
    options: AGETWO_OPTIONS
}

export const AgeTwo = (props?:Props) => {
    const {options} = props || DEFAULT_PROPS;

    return `
        <vertical-checkbox-list title="Which age group are you interested in?">
            ${mapToString(options, label => {
                return `<input-checkbox label="${label}"></input-checkbox>`
            })}

        </vertical-checkbox-list>
    `;
}

AgeTwo.args = DEFAULT_PROPS;

