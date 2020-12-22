import "@elements/images/image-label";
import {mockUiPath} from "~/mock/images";
import {Checkbox} from "~/components/input";
export default {
  title: 'Images/Label',
}

export const Label = ({path, label}) => {
    return `

    <img-ui path="${path}"></img-ui>
    <input-checkbox label="${label}"><input-checkbox/>
    `
}

Label.args = {
    path: mockUiPath,
    label:"Premium"
}

