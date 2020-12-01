import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, addClasses, appendTextLineId, addClassesId, setTextId} from "@utils/dom";
import {modulePage, ModulePageKind} from "@components/module";
import {mockThumbnail} from "@mock/images";
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
        addClassesId(widget, "recent", "hidden");
    } else {
        Array(4).fill(0).forEach((_, idx) => {
            const img = tmpl(recentThumbnailTmpl, {
                src: mockThumbnail
            });

            if(idx == 1) {
                addClasses(img, "jig-image-selected");
            }

            appendId(recent, "items", img);
        });
    }


    Array(10).fill(0).forEach((_, idx) => {
        const img = tmpl(resultThumbnailTmpl, {
            src: mockThumbnail
        });

        if(idx == 1 && showSelectedResult) {
            addClasses(img, ["row-span-3", "col-span-3"]);
        } else {
            addClasses(img, ["max-w-83.5", "max-h-83.5"]);
        }
        appendId(search, "items", img);
    });

    return widget
}