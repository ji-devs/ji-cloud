import {renderTemplate as tmpl} from "@utils/template";
import {mediaUi} from "@utils/path";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {modulePage, ModulePageKind} from "@components/module";
import {mockImageThumbnail} from "./mock-data";
import widgetTmpl from "@templates/_common/widgets/image-search/widget.html";
import resultThumbnailTmpl from "@templates/_common/widgets/image-search/result-thumbnail.html";
import recentThumbnailTmpl from "@templates/_common/widgets/image-search/recent-thumbnail.html";

interface SearchWidgetOptions {
    showRecent: boolean,
    showSelectedResult: boolean 
}
export const SearchWidget = ({showRecent, showSelectedResult}:SearchWidgetOptions) => {
    const widget = tmpl(widgetTmpl);

    const recent = getChildId(widget, "recent");
    const search = getChildId(widget, "search");

    if(!showRecent) {
        toggleClassesId(widget, "recent", "hidden", true);
    } else {
        Array(4).fill(0).forEach((_, idx) => {
            const img = tmpl(recentThumbnailTmpl, {
                src: mediaUi(mockImageThumbnail)
            });

            if(idx == 1) {
                toggleClasses(img, "jig-image-selected", true);
            }

            appendId(recent, "items", img);
        });
    }


    Array(10).fill(0).forEach((_, idx) => {
        const img = tmpl(resultThumbnailTmpl, {
            src: mediaUi(mockImageThumbnail)
        });

        if(idx == 1 && showSelectedResult) {
            toggleClasses(img, ["row-span-3", "col-span-3"], true);
        } else {
            toggleClasses(img, ["max-w-83.5", "max-h-83.5"], true);
        }
        appendId(search, "items", img);
    });

    return widget
}