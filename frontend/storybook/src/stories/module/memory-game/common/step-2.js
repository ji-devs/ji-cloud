import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {mockThemes} from "./mock-data";
import sidebarStep2ThemeItemSelected from "@templates/module/memory/edit/_common/sidebar/step-2-theme-item-selected.html";
import sidebarStep2ThemeItemDeselected from "@templates/module/memory/edit/_common/sidebar/step-2-theme-item-deselected.html";


export const appendStep2Sidebar = (page, {themeIndex}) => {
    mockThemes.forEach(({content, label, id}, idx) => {
        const item = idx === themeIndex ? tmpl(sidebarStep2ThemeItemSelected) : tmpl(sidebarStep2ThemeItemDeselected);
        const left = getChildId(item, "left");
        setTextId(left, "text-contents", content);

        setTextId(item, "label", label);

        toggleClasses(item, [`memory-theme-${id}`], true);
        
        appendId(page, "theme-items", item);
    });

    return page;
}
