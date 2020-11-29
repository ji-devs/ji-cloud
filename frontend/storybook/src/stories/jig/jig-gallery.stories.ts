import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jigcreatorone from "@templates/jig/gallery/jig-creator-one.html";

export default {
  title: 'JIG/Gallery',
}

export const JigCreatorOne = () =>
    tmpl(jigcreatorone, {

});