import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import imagelabel from "@templates/imagelabel/imagelabel.html";
import imageLabelFilter from "@templates/imagelabel/imagelabel-filter.html";
import labelcategory from "@templates/imagelabel/imagelabel-category.html";
import labelsummery from "@templates/imagelabel/imagelabel-summery.html";

export default {
  title: 'Image labeler',
}

export const ImageLabel = () =>
    tmpl(imagelabel, {
      navbarLink: "Label images",
    });

export const WithMenu = () => {
    const page = tmpl(imagelabel, {});
    const menu = tmpl(imageLabelFilter, {});

    appendId(page, "menu-container", menu)

    return page;
};

export const LabelCategory = () => {
  tmpl(labelcategory, {});

        return labelcategory;
};

export const LabelSummery = () => {
  tmpl(labelsummery, {});

        return labelsummery;
};

function setLabel(parentElement, label) {

    const element = tmpl(imageLabelFilter);
    // element.innerText = label;

    return appendId(parentElement, "label", element);
};
