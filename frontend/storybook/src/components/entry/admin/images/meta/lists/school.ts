import "@elements/core/lists/list-vertical";
import "@elements/core/inputs/checkbox";

import {mapToString} from "@utils/array";
import {SCHOOL_OPTIONS} from "~/mock/meta";


export default {
  title: 'Entry / Admin / Images / Meta / Lists',
}

interface Props {
  options: Array<String>
}

const DEFAULT_PROPS:Props = {
  options: SCHOOL_OPTIONS
}

export const ListHover = (props?:Props) => {
    
    const {options} = props || DEFAULT_PROPS;

    return `<vertical-full>
    ${mapToString(options, label => {
        return `<input-checkbox label="${label}"></input-checkbox>`
    })}    </vertical-full>`
}

ListHover.args = {
    label:"Placeholder"
}