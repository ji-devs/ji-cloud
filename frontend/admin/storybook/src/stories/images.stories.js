import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import checkbox from "@templates/_input/checkbox.html";
import imagesPage from "@templates/images/images-page.html";
import imageAdd from "@templates/images/image-add.html";
import imageEdit from "@templates/images/image-edit.html";
import imageFilter from "@templates/images/image-filter.html";
import imageFilterBubble from "@templates/images/image-filter-bubble.html";
import imageFilterOption from "@templates/images/image-filter-option.html";
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

    const pageContents = populateMetaOptions(tmpl(imageEdit));

    appendId(pageContainer, "page-contents", pageContents);

    return pageContainer;
}


export const WithMenu = () => {

    const pageContainer = tmpl(imagesPage);

    const pageContents = populateMetaOptions(tmpl(imageEdit));

    const menu = tmpl(imageFilter);

    ["Etz Chaim", "Rimon"]
      .forEach(label=> {
        appendId(menu, "bubbles", makeFilterBubble(label));
      });

    ["Etz Chaim", "Rimon", "Another School", "Some Place"]
      .forEach(label=> {
        appendId(menu, "options", makeFilterOption(label));
      });

    appendId(pageContents, "menu-container", menu);

    appendId(pageContainer, "page-contents", pageContents);

    return pageContainer;

};

function populateMetaOptions(container) {
  ["Clipart", "Photo", "B & W", "Drawing", "Comic"]
    .forEach(style => {
      appendId(container, "styles", makeCheckbox(style));
    });

  ["All ages", "Kindergarden", "Elementary", "Middle School", "High School", "University"]
    .forEach(age_range => {
      appendId(container, "age_ranges", makeCheckbox(age_range));
    });

  ["All", "No religion", "Reform/Conservative", "Orthodox", "Charedi"]
    .forEach(affiliation => {
      appendId(container, "affiliations", makeCheckbox(affiliation));
    });
  return container;
}

function makeFilterBubble(label) {
  return tmpl(imageFilterBubble, {label, id: label});
}

function makeFilterOption(label) {
  return tmpl(imageFilterOption, {label, id: label});
}

function makeCheckbox(label) {
  return tmpl(checkbox, {label, id: label});
}
/*
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
