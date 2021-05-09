import {ThemeKind} from "@elements/_themes/themes";

export const bgIconPath = (theme:ThemeKind):string => {
    return theme === "" 
        ? `core/_common/1px-white.png`
        : `theme/poster/${theme}/bg-icon.png`;
}
