import {ThemeKind, THEMES} from "@elements/module/_common/theme";

export const cardBackPath = (theme:ThemeKind):string => {
    return theme === "" 
        ? `core/_common/1px-white.png`
        : `theme/memory/${theme}/card-back.png`;
}
export const playerBackPath = (theme:ThemeKind):string => {
    return theme === "" 
        ? `module/memory/play/bg.png`
        : `module/memory/play/bg-${theme}.png`;
}
