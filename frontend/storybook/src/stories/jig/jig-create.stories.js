import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jigcreatorone from "@templates/admin/jig/jig-creator-one.html";
import jigaddmodule from "@templates/admin/jig/jig-add-module.html";

export default {
  title: 'JIG/Create',
}

export const JigCreatorOne = () =>
    tmpl(jigcreatorone, {

});

export const JigAddModule = () =>
    tmpl(jigaddmodule, {

});


