import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jigpublishone from "@templates/jig/publish/publish-one.html";
import jigpublishtooltip from "@templates/jig/publish/publish-error-tooltip.html";
import jiglanguagedropdown from "@templates/jig/publish/publish-language-dropdown.html";
import jigpublishage from "@templates/jig/publish/publish-age.html";
import jigpublishcategorydropdown from "@templates/jig/publish/publish-categories-dropdown.html";
import jigpublishcategories from "@templates/jig/publish/publish-selected-category.html";




export default {
  title: 'JIG/Publish',
}

export const JigPublishOne = () =>
    tmpl(jigpublishone, {

});

export const JigErrorTooltip = () =>  {
    const pageContainer = tmpl(jigpublishone);

    const pageContents = tmpl(jigpublishtooltip);

    appendId(pageContainer, "publish-error-tooltip", pageContents);

    return pageContainer;
}

export const JigAddLanguage = () =>  {
    const pageContainer = tmpl(jigpublishone);

    const pageContents = tmpl(jiglanguagedropdown);

    appendId(pageContainer, "publish-age-dropdown", pageContents);

    return pageContainer;
}

export const JigPublishAge = () =>  {
    const pageContainer = tmpl(jigpublishone);

    const pageContents = tmpl(jigpublishage);

    appendId(pageContainer, "publish-language-dropdown", pageContents);

    return pageContainer;
}

export const JigPublishCategoryDropdown = () =>  {
    const pageContainer = tmpl(jigpublishone);

    const pageContents = tmpl(jigpublishcategorydropdown);

    appendId(pageContainer, "publish-category-dropdown", pageContents);

    return pageContainer;
}

export const JigPublishCategories = () =>  {
    const pageContainer = tmpl(jigpublishone);

    const pageContents = tmpl(jigpublishcategories);

    appendId(pageContainer, "publish-categories", pageContents);

    return pageContainer;
}
