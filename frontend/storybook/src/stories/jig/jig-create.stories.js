import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jigcreatorone from "@templates/jig/jig-creator-one.html";
import jigaddmodule from "@templates/jig/jig-add-module.html";
import dropbox from "@templates/jig/jig-add-dropbox.html";
import jigdelete from "@templates/jig/jig-delete.html";
import jiglandingpage from "@templates/jig/jig-landingpage.html";
import jignavigation from "@templates/jig/jig-navigation.html";

export default {
  title: 'JIG/Create',
}

export const JigCreatorOne = () =>
    tmpl(jigcreatorone, {

});

export const JigAddModule = () =>
    tmpl(jigaddmodule, {

});

export const JigLandingPage = () =>
    tmpl(jiglandingpage, {

});

export const JigNavigation = () =>
    tmpl(jignavigation, {

});

export const JigCoverOne = () =>
    tmpl(jigcoverone, {

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
