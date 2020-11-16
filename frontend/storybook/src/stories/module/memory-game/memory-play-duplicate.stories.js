import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {startResizerOnElement} from "@utils/resize";
import {mockWords, mockThemes} from "./memory-common";
import modulePage from "@templates/module/_common/module-page.html";
import playerTmpl from "@templates/module/memory-game/play/player.html";
import cardPlayTmpl from "@templates/module/memory-game/play/memory-card.html";

export default {
  title: 'Modules/Memory-Game/Play/Duplicate',
}

const makePlayer = (nCards, nGrid) => () =>  mockPlayer(0, false, nCards, nGrid);
export const Play_8_Cards= makePlayer(8, 1);
export const Play_10_Cards = makePlayer(10, 2);
export const Play_12_Cards = makePlayer(12, 1);
export const Play_14_Cards = makePlayer(14);
export const Play_16_Cards = makePlayer(16, 1);
export const Play_18_Cards = makePlayer(18);
export const Play_20_Cards = makePlayer(20, 2);
export const Play_22_Cards = makePlayer(22);
export const Play_24_Cards = makePlayer(24, 3);
export const Play_26_Cards = makePlayer(26);
export const Play_28_Cards = makePlayer(28, 4);

export const Player_Theme_1 = () => mockPlayer(1, false, 12);
export const Player_Theme_1_Flipped = () => mockPlayer(1, true, 12);
//Helpers

function mockPlayer(selectedThemeIndex, isFlipped, nCards, nGrid) {
    const _page = appendId(tmpl(modulePage), "module-content", tmpl(playerTmpl));
    const page = appendMockCards(_page, {isFlipped, nCards});
    if(selectedThemeIndex) {
        const {id} = mockThemes[selectedThemeIndex];
        toggleClassesId(page, "cards", [`memory-theme-${id}`], true);
    }
    if(nGrid) {
        toggleClassesId(page, "cards", [`memory-grid-${nGrid}`], true);
    }

    startResizerOnElement(getChildId(page, "module-page"));

    return page;
}

function appendMockCards(page, {isFlipped, nCards}) {
    for(let i = 0; i < nCards; i++) {
        const word = mockWords[0];
        const card = tmpl(cardPlayTmpl);
        setTextId(card, "text-contents", word);
        appendId(page, "cards", card);

        if(isFlipped) {
            toggleClassesId(card, "flip", [`flip-card-clicked`], true);
        }
    }

    return page;
}
