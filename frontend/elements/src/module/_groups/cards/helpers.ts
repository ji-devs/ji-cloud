import {ThemeKind} from "@elements/_themes/themes";

export const cardBackPath = (theme:ThemeKind):string => {
    return theme === "" 
        ? `core/_common/1px-white.png`
        : `theme/module/_groups/cards/${theme}/card-back.png`;
}
export const playerBackPath = (theme:ThemeKind):string => {
    return theme === "" 
        ? `theme/module/_groups/cards/_default/bg.png`
        : `theme/module/_groups/cards/${theme}/bg.png`;
}
