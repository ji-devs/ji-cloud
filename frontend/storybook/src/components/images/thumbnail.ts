import "@elements/images/thumbnail";
import {mockThumbnail} from "~/mock/images";

export default {
  title: 'Images/Thumbnail',
}

export const Thumbnail = ({src}) => {
    return `<img-thumb src="${src}" />`
}

Thumbnail.args = {
    src: mockThumbnail,
}


Thumbnail.parameters = {
  docs: {
    description: {
      component: "this may eventually be replaced by `id` and `libraryKind` to match the Rust implementation"
    },
  },
};