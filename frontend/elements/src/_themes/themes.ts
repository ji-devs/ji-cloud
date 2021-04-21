export interface Theme {
    fontFamily1: string, 
    fontFamily2: string, 
    fontFamily3: string, 
    color1: string,
    color2: string,
    color3: string,
}

export type FontFamily = {
    en: string,
    he: string,
}

export type ThemeKind = "" | "chalkboard" | "happy-brush"

export const THEMES:Record<ThemeKind, Theme> = {
    [""]: {
        fontFamily1: "_sans",
        fontFamily2: "", 
        fontFamily3: "", 
        color1: "black",
        color2: "black",
        color3: "white",
    },
    chalkboard: {
        fontFamily1: `"Shesek - Regular", "Architects Daughter - Regular"`,
        fontFamily2: `"Frank Ruhl Libre - Medium", "Architects Daughter - Regular"`,
        fontFamily3: `"Shesek - Regular", "Roboto Slab - Regular"`,
        color1: "#272727",
        color2: "#AFCBF4",
        color3: "#D8E7FA",
    },
    ["happy-brush"]: {
        fontFamily1: `"Frank Ruhl Libre - Medium", "Roboto Slab - Medium"`,
        fontFamily2: `"Shesek - Regular", "Caveat - Medium"`,
        fontFamily3: "", 
        color1: "#272727",
        color2: "#FF6639",
        color3: "#FFF3ED",
    },
};
function setRootVars() {
    const style = document.documentElement.style;

    Object.entries(THEMES)
    .forEach(([id, theme]) => {
            const {fontFamily1, fontFamily2, fontFamily3, color1, color2, color3} = theme;
            style.setProperty(`--theme-${id}-color-1`, color1);
            style.setProperty(`--theme-${id}-color-2`, color2);
            style.setProperty(`--theme-${id}-color-3`, color3);
            style.setProperty(`--theme-${id}-font-family-1`, fontFamily1);
            style.setProperty(`--theme-${id}-font-family-2`, fontFamily2);
            style.setProperty(`--theme-${id}-font-family-3`, fontFamily3);
        });
}

setRootVars();
