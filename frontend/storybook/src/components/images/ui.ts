import "@elements/images/ui";
import {mockUiPath} from "~/mock/images";

export default {
  title: 'Images/Ui Image',
}

export const UiImage = ({path}) => {
    return `<img-ui path="${path}" />`
}

UiImage.args = {path: mockUiPath}