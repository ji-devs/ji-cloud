import "@elements/lists/list-hover";
import {Checkbox} from "~/components/input";
export default {
  title: 'Lists/List Hover',
}

export const ListHover = ({label}) => {
    return `<list-hover>
    <input-checkbox label="${label}" slot="one"></input-checkbox>
    </list-hover>`
}

ListHover.args = {
    label:"Placeholder"
}