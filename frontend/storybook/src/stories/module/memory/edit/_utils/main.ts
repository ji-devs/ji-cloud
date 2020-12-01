import {renderTemplate as tmpl} from "@utils/template";
import {appendId, setSrc, appendValueLineId, getChildId, setValueId, addClasses, appendTextLineId, addClassesId, setTextId, setSrcId} from "@utils/dom";

import {getInitialWords} from "../../_config/initial-words";
import {getThemeById, getThemeByIndex} from "../../_config/themes";
import {mockThumbnail} from "@mock/images";
import mainPairs from "@templates/module/memory/edit/_common/main/pairs.html";
import cardPairTextTextEdit from "@templates/module/memory/edit/_common/main/card-pairs/text-text-edit.html";
import cardPairTextTextPreview from "@templates/module/memory/edit/_common/main/card-pairs/text-text-preview.html";
import cardPairTextImageEdit from "@templates/module/memory/edit/_common/main/card-pairs/text-image-edit.html";
import cardPairTextImagePreview from "@templates/module/memory/edit/_common/main/card-pairs/text-image-preview.html";

interface MainPairOptions {
    pairKind: "text-text" | "text-image",
    flipSecond: boolean,
    isEdit: boolean,
    themeIndex: number
}

export function makeMainPairs({pairKind, flipSecond, isEdit, themeIndex}:MainPairOptions):Element {
    const el = tmpl(mainPairs);

    switch(pairKind) {
        case "text-text": 
            appendCardPairsTextText(el, {flipSecond, isEdit, themeIndex });
            break;
        case "text-image": 
            appendCardPairsTextImage(el, {flipSecond, isEdit, themeIndex });
            break;
    }

    return el;
}

interface TextPairOptions {
    flipSecond: boolean,
    isEdit: boolean,
    themeIndex: number
}
function appendCardPairsTextText(page:Element, {flipSecond, isEdit, themeIndex}:TextPairOptions) {
    getInitialWords().forEach(word => {
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
            addClasses(right, [`flip-card-clicked`]);
        }
        appendId(page, "cards", card);
    });


    const {id} = getThemeByIndex(themeIndex);
    addClassesId(page, "cards", [`memory-theme-${id}`]);

    return page;
}


function appendCardPairsTextImage(page:Element, {flipSecond, isEdit, themeIndex}:TextPairOptions):Element {
    getInitialWords().forEach((word, idx)  => {
        const pair = tmpl(isEdit ? cardPairTextImageEdit: cardPairTextImagePreview);
        const left = getChildId(pair, "left");
        if(isEdit) {
            setValueId(left, "text-contents", word);
        } else {
            setTextId(left, "text-contents", word);
        }
        const right = getChildId(pair, "right");
        
        if(flipSecond) {
            addClasses(right, [`flip-card-clicked`]);
        }
        
        if(idx % 2 == 0) {
            addClassesId(right, "image-waiting", "hidden");
            setSrcId(right, "image", mockThumbnail);
        } else {
            addClassesId(right, "image", "hidden");
        }
        appendId(page, "cards", pair);

    });


    const {id} = getThemeByIndex(themeIndex);
    addClassesId(page, "cards", [`memory-theme-${id}`]);

    return page;
}