import {ThemeKind, THEMES} from "@elements/module/_common/theme";

export const cardBackPath = (theme:ThemeKind):string => {
    return theme === "" 
        ? `core/_common/1px-transparent.png`
        : `theme/memory/${theme}/card-back.png`;
}
