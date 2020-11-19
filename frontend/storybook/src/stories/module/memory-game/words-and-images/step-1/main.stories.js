import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {MEDIA_UI} from "@utils/path";
import {appendCardPairsTextImage} from "../../common/card-pairs";
import step1Page from "@templates/module/memory/edit/words-and-images/step-1/step-1.html";

export default {
  title: 'Modules/Memory-Game/Edit/Words And Images/Step1/Main',
}


export const Images = () => {
    let page = tmpl(step1Page);
    toggleClassesId(page, "images-widget", "hidden", true);
    toggleClassesId(page, "text-widget", "hidden", true);

    appendCardPairsTextImage(page, {
      isTextEdit: false,
      flipSecond: false,
      themeIndex: 0
    });
    return page;
}