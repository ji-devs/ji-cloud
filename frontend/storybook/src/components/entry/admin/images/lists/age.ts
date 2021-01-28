import "@elements/core/lists/list-vertical";
import "@elements/core/inputs/checkbox";
import {AGE_OPTIONS} from "~/mock/meta";
import {mapToString} from "@utils/array";


export default {
  title: 'Admin / Images / Lists',
}
interface Props {
    options: Array<String>
}

const DEFAULT_PROPS:Props = {
    options: AGE_OPTIONS
}

export const Age = (props?:Props) => {
    const {options} = props || DEFAULT_PROPS;

    return `
        <list-vertical label="Suitable for age">
            ${mapToString(options, label => {
                return `<input-checkbox label="${label}"></input-checkbox>`
            })}

        </list-vertical>
    `;
}

Age.args = DEFAULT_PROPS;

