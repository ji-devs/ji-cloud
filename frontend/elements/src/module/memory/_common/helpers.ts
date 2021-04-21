import {ThemeKind} from "@elements/_themes/themes";

export const cardBackPath = (theme:ThemeKind):string => {
    return theme === "" 
        ? `core/_common/1px-white.png`
        : `theme/memory/${theme}/card-back.png`;
}
export const playerBackPath = (theme:ThemeKind):string => {
    return theme === "" 
        ? `theme/memory/_default/bg.png`
        : `theme/memory/${theme}/bg.png`;
}
