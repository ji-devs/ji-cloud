import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import imagelabel from "@templates/imagelabel/imagelabel.html";
import imageLabelFilter from "@templates/imagelabel/imagelabel-filter.html";

export default {
  title: 'Image labeler',
}

export const ImageLabel = () =>
    tmpl(imagelabel, {
      navbarLink: "Label images",
    });

    export const WithMenu = () => {
        const page = tmpl(imagelabel, {});

        const element = setLabel(tmpl(imageLabelFilter), "with menu");
        appendId(page, "list", toggleClassesId(element, "menu", ["hidden"], false));

        return page;
    };

    function setLabel(parentElement, label) {

        const element = tmpl(imageLabelfilter);
        element.innerText = label;

        return appendId(parentElement, "label", element);
    };
