import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jigtable from "@templates/jig/jig-table.html";
import jigtooltip from "@templates/jig/jig-copyemail.html";
import jigprivacy from "@templates/jig/jig-privacy.html";
import jigfilter from "@templates/jig/jig-filter.html";


export default {
  title: 'JIG',
}


export const JigTable = () =>
    tmpl(jigtable, {

});




export const JigTooltip = () =>  {
    const pageContainer = tmpl(jigtable);

    const pageContents = tmpl(jigtooltip);

    appendId(pageContainer, "tooltip", pageContents);

    return pageContainer;
}

export const JigPrivacy = () =>  {
    const pageContainer = tmpl(jigtable);

    const pageContents = tmpl(jigprivacy);

    appendId(pageContainer, "privacy-dropdown", pageContents);

    return pageContainer;
}

export const JigFilter = () =>  {
    const pageContainer = tmpl(jigtable);

    const pageContents = tmpl(jigfilter);

    appendId(pageContainer, "jig-filter", pageContents);

    return pageContainer;
}
