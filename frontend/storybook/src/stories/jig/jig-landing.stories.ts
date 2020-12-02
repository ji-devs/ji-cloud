import {renderTemplate as tmpl} from "@utils/template";
import jiglandingpage from "@templates/jig/landing/jig-landingpage.html";

export default {
  title: 'JIG/Landing',
}

export const JigLandingPage = () =>
    tmpl(jiglandingpage, {

});