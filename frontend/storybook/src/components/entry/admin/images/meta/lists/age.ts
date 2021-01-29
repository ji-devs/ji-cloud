import "@elements/core/lists/list-vertical";
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

const STR_AGESUITABLE = "Suitable for age";

export const Age = (props?:Props) => {
    const {options} = props || DEFAULT_PROPS;

    return `
        <list-vertical label="${STR_AGESUITABLE}">
            ${mapToString(options, label => {
                return `<input-checkbox label="${label}"></input-checkbox>`
            })}

        </list-vertical>
    `;
}

Age.args = DEFAULT_PROPS;

