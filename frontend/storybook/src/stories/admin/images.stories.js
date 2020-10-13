import {renderTemplate as tmpl, renderDivText} from "@utils/template";
import {appendId, toggleClassesId, setTextId, setValueId, setAttributeId} from "@utils/dom";
import {MEDIA_UI} from "@utils/path";
import checkbox from "@templates/_common/input/checkbox.html";
import imagesPage from "@templates/admin/images/images-page.html";
import imageAdd from "@templates/admin/images/image-add.html";
import imageEdit from "@templates/admin/images/image-edit.html";
import imageEditMeta from "@templates/admin/images/image-edit-meta.html";
import imageEditCategories from "@templates/admin/images/image-edit-categories.html";
import imageEditCategoriesParent from "@templates/admin/images/image-edit-categories-parent.html";
import imageEditCategoriesParentEnd from "@templates/admin/images/image-edit-categories-parent-end.html";
import imageEditCategoriesChild from "@templates/admin/images/image-edit-categories-child.html";
import imageEditCategoriesChildEnd from "@templates/admin/images/image-edit-categories-child-end.html";
import imageEditCategoriesSumParent from "@templates/admin/images/image-edit-categories-sum-parent.html";
import imageEditCategoriesSumChild from "@templates/admin/images/image-edit-categories-sum-child.html";
import imageFilter from "@templates/admin/images/image-filter.html";
import imageFilterBubble from "@templates/admin/images/image-filter-bubble.html";
import imageFilterOption from "@templates/admin/images/image-filter-option.html";
import imageOverview from "@templates/admin/images/image-edit-overview.html";
import imagesSearch from "@templates/admin/images/images-search.html";
import imageGridItemRed from "@templates/_common/image/image-grid-item-red.html";
import imageGridItemGreen from "@templates/_common/image/image-grid-item-green.html";

export default {
  title: 'Admin/Images',
}

export const Add = () =>  {
    const pageContainer = tmpl(imagesPage);

    const pageContents = tmpl(imageAdd);

    appendId(pageContainer, "page-contents", pageContents);

    return pageContainer;
}
export const Edit = () =>  {
    const pageContainer = tmpl(imagesPage);
    const editContainer = tmpl(imageEdit);
    const editMeta = tmpl(imageEditMeta);

    appendId(editContainer, "right-area", editMeta);

    setAttributeId(editContainer, "img", "src", `${MEDIA_UI}/red-sea-book.png`);

    setTextId(editContainer, "next", "Next");

    const pageContents = populateMetaOptions(editContainer);

    appendId(pageContainer, "page-contents", pageContents);

    return pageContainer;
}
export const Categories = () =>  {
    const pageContainer = tmpl(imagesPage);
    const editContainer = tmpl(imageEdit);
    setAttributeId(editContainer, "img", "src", `${MEDIA_UI}/red-sea-book.png`);
    setTextId(editContainer, "next", "Next");
    const editCategories = tmpl(imageEditCategories);

    const cat1 = tmpl(imageEditCategoriesParentEnd, {name: "English"});

    const cat2 = tmpl(imageEditCategoriesParent, {name: "Hebrew"});
    toggleClassesId(cat2, "arrow", ["transform","rotate-90","-m-1"], true);

    const cat2Child1 = tmpl(imageEditCategoriesChild, {name: "Vocabulary"});
    toggleClassesId(cat2Child1, "arrow", ["transform","rotate-90","-m-1"], true);
    const cat2Child1Child = tmpl(imageEditCategoriesChildEnd, {name: "Blah"});
    const cat2Child2 = tmpl(imageEditCategoriesChildEnd, {name: "Parsha"});
    const cat2Child3 = tmpl(imageEditCategoriesChild, {name: "Shapes"});
    appendId(cat2Child1, "children", cat2Child1Child);
    [cat2Child1, cat2Child2, cat2Child3].forEach(x => appendId(cat2, "children", x));

    const cat3 = tmpl(imageEditCategoriesParent, {name: "Spanish"});

    [cat1, cat2, cat3].forEach(cat => appendId(editCategories, "select-list", cat));

    const sum1 = tmpl(imageEditCategoriesSumParent, {name: "English"});
    const sum2 = tmpl(imageEditCategoriesSumParent, {name: "Hebrew"});
    const sum2Child1 = tmpl(imageEditCategoriesSumChild, {name: "Vocabulary"});
    const sum2Child1Child = tmpl(imageEditCategoriesSumChild, {name: "Blah"});

    appendId(sum2Child1, "children", sum2Child1Child);
    appendId(sum2, "children", sum2Child1);

    [sum1, sum2].forEach(sum => appendId(editCategories, "summary-list", sum));

    appendId(editContainer, "right-area", editCategories);

    const pageContents = editContainer;

    appendId(pageContainer, "page-contents", pageContents);

    return pageContainer;
}
export const Overview = () => {
    const pageContainer = tmpl(imagesPage);
    const editContainer = tmpl(imageEdit);
    setAttributeId(editContainer, "img", "src", `${MEDIA_UI}/red-sea-book.png`);
    setTextId(editContainer, "next", "Publish");
    const overview = tmpl(imageOverview, {
      name: "Moses parts the Nile",
      description: "An open book, Moses hold his stick and raise his hands up, and part the Nile. An open book, Moses hold his stick and raise his hands up, and part the Nile. An open book, Moses hold his stick and raise his hands up, and part the Nile."
    });
    ["Clipart", "Photo", "B & W", "Drawing", "Comic"]
      .forEach(style => {
        appendId(overview, "styles", renderDivText(style));
      });
    ["All ages", "Kindergarden", "Elementary", "Middle School", "High School", "University"]
      .forEach(age_range => {
        appendId(overview, "age_ranges", renderDivText(age_range));
      });

    ["All", "No religion", "Reform/Conservative", "Orthodox", "Charedi"]
      .forEach(affiliation => {
        appendId(overview, "affiliations", renderDivText(affiliation));
      });


    const sum1 = tmpl(imageEditCategoriesSumParent, {name: "English"});
    const sum2 = tmpl(imageEditCategoriesSumParent, {name: "Hebrew"});
    const sum2Child1 = tmpl(imageEditCategoriesSumChild, {name: "Vocabulary"});
    const sum2Child1Child = tmpl(imageEditCategoriesSumChild, {name: "Blah"});

    appendId(sum2Child1, "children", sum2Child1Child);
    appendId(sum2, "children", sum2Child1);

    [sum1, sum2].forEach(sum => appendId(overview, "category-summaries", sum));

    appendId(editContainer, "right-area", overview);

    const pageContents = editContainer ;

    appendId(pageContainer, "page-contents", pageContents);

    return pageContainer;
};

export const Search = () => {
    const pageContainer = tmpl(imagesPage);
    const searchContainer = tmpl(imagesSearch);

    setTextId(searchContainer, "n-results", "42");
    setValueId(searchContainer, "page", "9");

    [
      ["red-sea-book.png", "Red Sea"],
      ["red-sea-book.png", "Red Sea"]
    ].forEach(([filename, label], idx) => {
      const id = idx;
      const template = idx % 2 == 0 ? imageGridItemGreen : imageGridItemRed;
      const src = `${MEDIA_UI}/${filename}`;
      const img = tmpl(template, {id: id, label, src});
      appendId(searchContainer, "grid", img);
    });

    const pageContents = searchContainer;

    appendId(pageContainer, "page-contents", pageContents);

    return pageContainer;
}
/*
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
*/

function populateMetaOptions(container) {
  ["Clipart", "Photo", "B & W", "Drawing", "Comic", "frame", "Icon / Emoji / Button", "Word / Label", "Layout"]
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
