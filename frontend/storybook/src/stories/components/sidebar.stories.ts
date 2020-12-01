import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import components from "@templates/_demo/components.html";
import sidebar from "@templates/_demo/sidebar.html";
import {SearchWidget} from "@components/image-search";
export default {
  title: 'Components/Sidebar',
}

export const Images = () => {
    const page = tmpl(sidebar);
    const widget = SearchWidget({showRecent: false, showSelectedResult: false});
    appendId(page, "sidebar", widget);

    return page;

}
export const Images_Recent = () => {
    const page = tmpl(sidebar);
    const widget = SearchWidget({showRecent: true, showSelectedResult: false});
    appendId(page, "sidebar", widget);

    return page;
}
export const Images_KitchenSink = () => {
    const page = tmpl(sidebar);
    const widget = SearchWidget({showRecent: true, showSelectedResult: true});
    appendId(page, "sidebar", widget);

    return page;
}