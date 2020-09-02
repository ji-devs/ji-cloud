import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import imagesPage from "@templates/images/images-page.html";
import imageAdd from "@templates/images/image-add.html";
import imageEdit from "@templates/images/image-edit.html";
import imageFilter from "@templates/images/image-filter.html";
import imageCategory from "@templates/images/image-category.html";
import imageSummary from "@templates/images/image-summary.html";

export default {
  title: 'Image labeler',
}

export const Add = () =>  {
    const pageContainer = tmpl(imagesPage);

    const pageContents = tmpl(imageAdd);

    appendId(pageContainer, "page-contents", pageContents);

    return pageContainer;
}
export const Edit = () =>  {
    const pageContainer = tmpl(imagesPage);

    const pageContents = tmpl(imageEdit);

    appendId(pageContainer, "page-contents", pageContents);

    return pageContainer;
}

/*

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

export const ImageSummary = () => {
  tmpl(imagesummary, {});

        return imagesummary;
};

function setLabel(parentElement, label) {

    const element = tmpl(imageLabelFilter);
    // element.innerText = label;

    return appendId(parentElement, "label", element);
};
*/
