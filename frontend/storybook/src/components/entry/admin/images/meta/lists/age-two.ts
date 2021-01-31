import "@elements/core/dividers/vertical-full";
import "@elements/core/inputs/checkbox";
import {AGE_OPTIONS} from "~/mock/meta";
import {mapToString} from "@utils/array";


export default {
  title: 'Entry / Admin / Images / Meta / Lists',
}
interface Props {
    options: Array<String>
}

const DEFAULT_PROPS:Props = {
    options: AGE_OPTIONS
}

const STR_AGETITLE ="Which age group are you interested in?";

export const AgeTwo = (props?:Props) => {
    const {options} = props || DEFAULT_PROPS;

    return `
        <vertical-full title="${STR_AGETITLE}">
            ${mapToString(options, label => {
                return `<input-checkbox label="${label}"></input-checkbox>`
            })}

        </vertical-full>
    `;
}

AgeTwo.args = DEFAULT_PROPS;

