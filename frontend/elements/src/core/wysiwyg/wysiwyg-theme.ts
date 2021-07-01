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
    const themeInfo = THEMES[theme];

    const fontSize = themeInfo.textEditor[el].fontSize;
    const font = (themeInfo as any)["fontFamily" + themeInfo.textEditor[el].fontFamily];
    let color = (themeInfo as any)["color" + themeInfo.textEditor[el].fontColor];
    
    // the following are both doing same thing
    // color = `#${color[0].toString(16)}${color[1].toString(16)}${color[2].toString(16)}`;
    color = color.reduce((accumulator: string, currentValue: number) => {
        return accumulator += currentValue.toString(16);
    }, "#")

    return [
        [`--${el}-color`, color],
        [`--${el}-font`, font],
        [`--${el}-font-size`, `${fontSize}px`],
    ];
}
