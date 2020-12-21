import "@elements/images/image-label";
import {mockThumbnail} from "~/mock/images";
import {Checkbox} from "~/components/input";
export default {
  title: 'Images/Label',
}

export const Label = ({src, label}) => {
    return `

    <img-thumb src="${src}" />
    <input-checkbox label="${label}"><input-checkbox/>
    `
}

Label.args = {
    path: mockThumbnail,
    label:"Premium Image"
}

