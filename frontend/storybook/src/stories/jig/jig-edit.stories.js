import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
//these are the Add templates
import jigaddmodule from "@templates/jig/edit/jig-add-module.html";
import dropbox from "@templates/jig/edit/jig-add-dropbox.html";
import jigdelete from "@templates/jig/edit/jig-delete.html";

export default {
  title: 'JIG/Create',
}

export const JigAddModule = () =>
    tmpl(jigaddmodule, {

});

export const Dropbox = () =>  {
    const pageContainer = tmpl(jigaddmodule);

    const pageContents = tmpl(dropbox);

    appendId(pageContainer, "jig-dropbox", pageContents);

    return pageContainer;
}

export const JigDelete = () =>  {
    const pageContainer = tmpl(jigaddmodule);

    const pageContents = tmpl(jigdelete);

    appendId(pageContainer, "jigdelete", pageContents);

    return pageContainer;
}
