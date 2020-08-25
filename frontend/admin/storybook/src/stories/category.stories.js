import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import categoriesPage from "@templates/categories/categories-page.html";
import categoryMainSelected from "@templates/categories/category_main_selected.html";
import categoryMainDeselected from "@templates/categories/category_main_deselected.html";
import categorySub from "@templates/categories/category_sub.html";

export default {
  title: 'Categories',
}

export const SingleItem = () => {
    const page = tmpl(categoriesPage, {});
    
    appendId(page, "list", tmpl(categoryMainDeselected, {name: "deselected"}));
    appendId(page, "list", tmpl(categoryMainSelected, {name: "selected"}));
    return page;
}

export const MultiItem = () => {
    const page = tmpl(categoriesPage, {});
    appendId(page, "list", createTree(false));
    appendId(page, "list", createTree(true));
    return page;
}

export const WithMenu = () => {
    const page = tmpl(categoriesPage, {});

    appendId(page, "list", toggleClassesId(tmpl(categoryMainSelected, {name: "with menu"}), "menu", ["hidden"], false));

    return page;
}

function createTree(selected) {
    const subItems = [
        tmpl(categorySub, {name: "sub item 1"}),
        tmpl(categorySub, {name: "sub item 2"}),
    ];

    subItems.forEach(subItem => {
        const subSubItems = [
            tmpl(categorySub, {name: "sub item A"}),
            tmpl(categorySub, {name: "sub item B"}),
        ];
        subSubItems.forEach(subSubItem => {
            appendId(subItem, "children", subSubItem);
        });
    });

    const mainItem = selected 
        ? tmpl(categoryMainSelected, {name: "selected"})
        : tmpl(categoryMainDeselected, {name: "deselected"});

    subItems.forEach(subItem=> {
        appendId(mainItem, "children", subItem);
    });

    return mainItem;
}
