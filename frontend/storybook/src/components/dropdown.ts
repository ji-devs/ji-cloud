import "@elements/dropdowns/selected-dropdown";
import {ListHover} from "~/components/lists/list-hover";
export default {
  title: 'Dropdown',
}


export const SelectedDropdown = () => {
    return `<selected-dropdown label="Title">
      <list-hover/>
    </selected-dropdown>
`
}

