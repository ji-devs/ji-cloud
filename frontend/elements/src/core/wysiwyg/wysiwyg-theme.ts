import { THEMES, ThemeKind } from "@elements/_themes/themes";
import { ElementType } from "./wysiwyg-types";

export function getThemeVars(theme: ThemeKind): [string, string][] {
    let arr = getElTheme(ElementType.H1, theme);
    arr = arr.concat(getElTheme(ElementType.H2, theme));
    arr = arr.concat(getElTheme(ElementType.P1, theme));
    arr = arr.concat(getElTheme(ElementType.P2, theme));
    return arr;
}

function getElTheme(elName: ElementType, theme: ThemeKind): [string, string][] {
    const el = elName.toLowerCase() as 'h1' | 'h2' | 'p1' | 'p2';
    const themeInfo = THEMES[theme];

    const fontSize = themeInfo[el].fontSize;
    const color = (themeInfo as any)["color" + themeInfo[el].fontColor];
    const font = (themeInfo as any)["fontFamily" + themeInfo[el].fontFamily];

    return [
        [`--${el}-color`, color],
        [`--${el}-font`, font],
        [`--${el}-font-size`, `${fontSize}px`],
    ];
}
