import {ThemeKind} from "@elements/_themes/themes";

export const themeIconPath = (theme:ThemeKind, hover:boolean):string => {
    //return theme === "" ? `core/_common/1px-white.png`
    return theme === "" ? `theme/module/_groups/design/_default/icon${hover ? "-hover" : ""}.jpg`
        : `theme/module/_groups/design/${theme}/icon${hover ? "-hover" : ""}.jpg`;
}