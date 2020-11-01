import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import memorycreator from "@templates/memory/memory-creator.html";
import memorystepone from "@templates/memory/memory-step-one.html";
import addword from "@templates/memory/add-word-tooltip.html";
import worderror from "@templates/memory/memory-error.html";
import worderror from "@templates/memory/memory-step-two.html";


export default {
  title: 'Games/memory',
}

export const Memory = () =>
    tmpl(memorycreator, {

});

export const MemoryStepOne = () =>
    tmpl(memorystepone, {

});

export const MemoryStepTwo = () =>
    tmpl(memorysteptwo, {

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
