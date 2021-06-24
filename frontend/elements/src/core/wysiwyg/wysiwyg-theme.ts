import { THEMES, ThemeKind, TextEditor as TextEditorTheme } from "@elements/_themes/themes";
import { ElementType } from "./wysiwyg-types";

export function getThemeVars(theme: ThemeKind): [string, string][] {
    let arr = getElTheme(ElementType.H1, theme);
    arr = arr.concat(getElTheme(ElementType.H2, theme));
    arr = arr.concat(getElTheme(ElementType.P1, theme));
    arr = arr.concat(getElTheme(ElementType.P2, theme));
    return arr;
}

function getElTheme(elName: ElementType, theme: ThemeKind): [string, string][] {
    const el:keyof TextEditorTheme= elName.toLowerCase() as any;
    const themeInfo = THEMES[theme].textEditor;

    const fontSize = themeInfo[el].fontSize;
    const color = (themeInfo as any)["color" + themeInfo[el].fontColor];
    const font = (themeInfo as any)["fontFamily" + themeInfo[el].fontFamily];

    return [
        [`--${el}-color`, color],
        [`--${el}-font`, font],
        [`--${el}-font-size`, `${fontSize}px`],
    ];
}
