import "@elements/core/lists/list-vertical";
import "@elements/core/inputs/checkbox";
export default {
  title: 'Entry / Admin / Images / Lists',
}

export const ListHover = ({label}) => {
    return `<vertical-full>
    <input-checkbox label="${label}" slot="one"></input-checkbox>
    </vertical-full>`
}

ListHover.args = {
    label:"Placeholder"
}