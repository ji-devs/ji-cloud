import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jiglandingpage from "@templates/jig/landing/jig-landingpage.html";

export default {
  title: 'JIG/Landing',
}

export const JigLandingPage = () =>
    tmpl(jiglandingpage, {

});