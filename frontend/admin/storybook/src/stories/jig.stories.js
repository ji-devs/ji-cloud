import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import jigtable from "@templates/jig/jig-table.html";
import jigtooltip from "@templates/jig/jig-copyemail.html";

export default {
  title: 'JIG',
}


export const JigTable = () =>
    tmpl(jigtable, {

});

export const JigTooltip = () =>
    tmpl(jigtooltip, {

});
