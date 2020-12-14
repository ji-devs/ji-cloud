import "@elements/images/ui-image";
import {mockThumbnail} from "~/mock/images";

export default {
  title: 'Images/Ui Image',
}

export const UiImage = () => {
    return `<ui-image path="${mockThumbnail}" />`
}