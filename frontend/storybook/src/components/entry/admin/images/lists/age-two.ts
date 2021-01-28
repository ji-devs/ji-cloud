import "@elements/core/dividers/vertical-full";
import "@elements/core/inputs/checkbox";
import {AGE_OPTIONS} from "~/mock/meta";
import {mapToString} from "@utils/array";


export default {
  title: 'Lists/List',
}
interface Props {
    options: Array<String>
}

const DEFAULT_PROPS:Props = {
    options: AGE_OPTIONS
}

export const AgeTwo = (props?:Props) => {
    const {options} = props || DEFAULT_PROPS;

    return `
        <vertical-full title="Which age group are you interested in?">
            ${mapToString(options, label => {
                return `<input-checkbox label="${label}"></input-checkbox>`
            })}

        </vertical-full>
    `;
}

AgeTwo.args = DEFAULT_PROPS;

