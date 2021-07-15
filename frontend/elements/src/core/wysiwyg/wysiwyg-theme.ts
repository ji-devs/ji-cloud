import { THEMES, ThemeId, TextEditor as TextEditorTheme, TextEditorVariant } from "@elements/_themes/themes";
import { ElementType } from "./wysiwyg-types";

export function getThemeVars(theme: ThemeId): [string, string][] {
    let arr = getElTheme(ElementType.H1, theme);
    arr = arr.concat(getElTheme(ElementType.H2, theme));
    arr = arr.concat(getElTheme(ElementType.P1, theme));
    arr = arr.concat(getElTheme(ElementType.P2, theme));
    return arr;
}

function getElTheme(elName: ElementType, theme: ThemeId): [string, string][] {
    const el:keyof TextEditorTheme= elName.toLowerCase() as any;
    const themeInfo = THEMES[theme];
    const themeVariant = (themeInfo.textEditor[el] as TextEditorVariant);

    const fontSize = themeVariant.fontSize;
    const font = themeInfo.fontFamilies[themeVariant.fontFamily];
    const color = themeInfo.colors[themeVariant.fontColor];
   
    return [
        [`--${el}-color`, color],
        [`--${el}-font`, font],
        [`--${el}-font-size`, `${fontSize}rem`],
    ];
}
