import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {mockWords, mockThemes} from "./mock-data";
import cardPairTextTextEdit from "@templates/module/memory/edit/_common/card-pairs/text-text-edit.html";
import cardPairTextTextPreview from "@templates/module/memory/edit/_common/card-pairs/text-text-preview.html";
import cardPairTextImageEdit from "@templates/module/memory/edit/_common/card-pairs/text-image-edit.html";
import cardPairTextImagePreview from "@templates/module/memory/edit/_common/card-pairs/text-image-preview.html";
import cardPlayTmpl from "@templates/module/memory/play/memory-card.html";

interface TextTextPairOptions {
    flipSecond: boolean,
    isEdit: boolean,
    themeIndex: number
}
export function appendCardPairsTextText(page:Element, {flipSecond, isEdit, themeIndex}:TextTextPairOptions) {
    mockWords.forEach(word => {
        const card = tmpl(isEdit ? cardPairTextTextEdit : cardPairTextTextPreview);
        const left = getChildId(card, "left");
        if(isEdit) {
            setValueId(left, "text-contents", word);
        } else {
            setTextId(left, "text-contents", word);
        }
        const right = getChildId(card, "right");
        if(isEdit) {
            setValueId(right, "text-contents", word);
        } else {
            setTextId(right, "text-contents", word);
        }
        
        if(flipSecond) {
            toggleClasses(right, [`flip-card-clicked`], true);
        }
        appendId(page, "cards", card);
    });


    const {id} = mockThemes[themeIndex];
    toggleClassesId(page, "cards", [`memory-theme-${id}`], true);

    return page;
}


interface TextImagePairOptions {
    flipSecond: boolean,
    isTextEdit: boolean,
    themeIndex: number
}
export function appendCardPairsTextImage(page:Element, {flipSecond, isTextEdit, themeIndex}:TextImagePairOptions):Element {
    mockWords.forEach(word => {
        const card = tmpl(isTextEdit ? cardPairTextImageEdit: cardPairTextImagePreview);
        const left = getChildId(card, "left");
        if(isTextEdit) {
            setValueId(left, "text-contents", word);
        } else {
            setTextId(left, "text-contents", word);
        }
        const right = getChildId(card, "right");
        
        if(flipSecond) {
            toggleClasses(right, [`flip-card-clicked`], true);
        }
        appendId(page, "cards", card);
    });


    const {id} = mockThemes[themeIndex];
    toggleClassesId(page, "cards", [`memory-theme-${id}`], true);

    return page;
}
