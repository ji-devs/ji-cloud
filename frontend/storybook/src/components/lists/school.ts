import "@elements/lists/list-hover";
import {Checkbox} from "~/components/input";
import {mapToString} from "@utils/array";
import {SCHOOL_OPTIONS} from "~/mock/meta";


export default {
  title: 'Lists/List',
}

interface Props {
  options: Array<String>
}

const DEFAULT_PROPS:Props = {
  options: SCHOOL_OPTIONS
}

export const ListHover = (props?:Props) => {
    
    const {options} = props || DEFAULT_PROPS;

    return `<list-hover>
    ${mapToString(options, label => {
        return `<input-checkbox label="${label}"></input-checkbox>`
    })}    </list-hover>`
}

ListHover.args = {
    label:"Placeholder"
}