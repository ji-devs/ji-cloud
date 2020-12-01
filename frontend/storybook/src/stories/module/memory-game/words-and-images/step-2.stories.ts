import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {MEDIA_UI} from "@utils/path";
import step2Page from "@templates/module/memory/edit/words-and-images/step-2.html";
import {appendCardPairsTextImage} from "../common/card-pairs";
import { appendStep2Sidebar } from "../common/step-2";

export default {
  title: 'Modules/Memory-Game/Edit/Words And Images/Step2',
}


export const Step2 = () => mockStep2(tmpl(step2Page), 0);

export const Step2_Theme_1 = () => mockStep2(tmpl(step2Page), 1);
export const Step2_Theme_1_FlipSecond = () => mockStep2(tmpl(step2Page), 1, true);
export const Step2_Theme_2 = () => mockStep2(tmpl(step2Page), 2);
export const Step2_Theme_2_FlipSecond = () => mockStep2(tmpl(step2Page), 2, true);


function mockStep2(_page, themeIndex, flipSecond) {
    const page = appendCardPairsTextImage(_page, {
        flipSecond, 
        isEdit: false,
        themeIndex
    });

    return appendStep2Sidebar(page, {themeIndex});
}