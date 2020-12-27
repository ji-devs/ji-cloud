import "@elements/lists/checkbox-list";
import "@elements/inputs/checkbox";
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

export const Age = (props?:Props) => {
    const {options} = props || DEFAULT_PROPS;

    return `
        <checkbox-list title="Suitable for age">
            ${mapToString(options, label => {
                return `<input-checkbox label="${label}"></input-checkbox>`
            })}

        </checkbox-list>
    `;
}

Age.args = DEFAULT_PROPS;

