import "@elements/core/lists/list-vertical";
import "@elements/core/inputs/checkbox";
export default {
  title: 'Entry / Admin / Images / Meta / Lists',
}

export const ListHoverItem = ({label}) => {
    return `<vertical-full>
    <input-checkbox label="${label}" slot="one"></input-checkbox>
    </vertical-full>`
}

ListHoverItem.args = {
    label:"Placeholder"
}