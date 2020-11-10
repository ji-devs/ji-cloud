import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import modeChoosePage from "@templates/module/memory-game/edit/mode-choose-page.html";
import memorystepone from "@templates/module/memory-game/edit/memory-step-one.html";
import addword from "@templates/module/memory-game/edit/add-word-tooltip.html";
import worderror from "@templates/module/memory-game/edit/memory-error.html";
import memorysteptwo from "@templates/module/memory-game/edit/memory-step-two.html";
import memorystepthree from "@templates/module/memory-game/edit/memory-step-three.html";
import memorystepfour from "@templates/module/memory-game/edit/memory-step-four.html";
import memorysuccess from "@templates/module/memory-game/edit/memory-success.html";
import memorysummary from "@templates/module/memory-game/edit/memory-summary.html";



export default {
  title: 'Modules/Memory-Game/Edit',
}

export const ChooseMode = () =>
    tmpl(modeChoosePage, {

});

/*
export const MemoryStepOne = () =>
    tmpl(memorystepone, {

});

export const MemoryStepTwo = () =>
    tmpl(memorysteptwo, {

});

export const MemoryStepThree = () =>
    tmpl(memorystepthree, {

});

export const MemoryStepFour = () =>
    tmpl(memorystepfour, {

});

export const MemorySuccess = () =>
    tmpl(memorysuccess, {

});
export const MemorySummary = () =>
    tmpl(memorysummary, {

});

export const AddWord = () =>  {
    const pageContainer = tmpl(memorystepone);

    const pageContents = tmpl(addword);

    appendId(pageContainer, "add-word-tooltip", pageContents);

    return pageContainer;
}

export const WordError = () =>  {
    const pageContainer = tmpl(memorystepone);

    const pageContents = tmpl(worderror);

    appendId(pageContainer, "memory-error", pageContents);

    return pageContainer;
}
*/