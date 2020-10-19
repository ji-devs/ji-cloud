import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jignavigation from "@templates/jig/design/jig-navigation.html";
import jigcoverone from "@templates/jig/design/jig-cover1.html";
import jigaddimage from "@templates/jig/design/cover-add-image.html";

export default {
  title: 'JIG/Design',
}

export const JigNavigation = () =>
    tmpl(jignavigation, {

});

export const JigCoverOne = () =>
    tmpl(jigcoverone, {

});

export const JigAddImage = () =>
    tmpl(jigaddimage, {

});