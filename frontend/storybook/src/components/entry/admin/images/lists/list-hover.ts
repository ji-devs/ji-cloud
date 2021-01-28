import "@elements/core/lists/list-vertical";
import "@elements/core/inputs/checkbox";
export default {
  title: 'Lists/List',
}

export const ListHover = ({label}) => {
    return `<vertical-full>
    <input-checkbox label="${label}" slot="one"></input-checkbox>
    </vertical-full>`
}

ListHover.args = {
    label:"Placeholder"
}