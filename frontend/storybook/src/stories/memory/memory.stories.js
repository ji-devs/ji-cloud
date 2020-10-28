import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import memorycreator from "@templates/memory/memory-creator.html";

export default {
  title: 'Games/memory',
}

export const Memory = () =>
    tmpl(memorycreator, {

});
