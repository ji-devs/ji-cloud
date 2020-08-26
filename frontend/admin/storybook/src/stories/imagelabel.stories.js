import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import imagelabel from "@templates/imagelabel.html";

export default {
  title: 'Image labeler',
}

export const ImageLabel = () =>
    tmpl(imagelabel, {
      navbarLink: "Label images",
    });
