export interface Theme {
    fontFamily1: string, 
    fontFamily2: string, 
    fontFamily3: string, 
    color1: string,
    color2: string,
    color3: string,
    h1: ThemeText,
    h2: ThemeText,
    p1: ThemeText,
    p2: ThemeText,
}

export interface ThemeText {
    fontFamily: string, //mapping
    fontSize: number //direct value
    fontColor: string //mapping
}
export type FontFamily = {
    en: string,
    he: string,
}

export type ThemeKind = "blank" | "chalkboard" | "happy-brush"

export const STR_THEME_LABEL:Record<ThemeKind, string> = {
    "blank": "Blank",
    "chalkboard": "Chalkboard",
    "happy-brush": "Happy Brush"
}

//TODO - import the config JSON via typescript / rollup
//to match themes.rs
export const THEMES:Record<ThemeKind, Theme> = {
    blank: {
        fontFamily1: "_sans",
        fontFamily2: "", 
        fontFamily3: "", 
        color1: "black",
        color2: "black",
        color3: "white",
        h1: {
            fontFamily: "1",
            fontSize: 28,
            fontColor: "2",
        },
        h2: {
            fontFamily: "2",
            fontSize: 25,
            fontColor: "3",
        },
        p1: {
            fontFamily: "1",
            fontSize: 14,
            fontColor: "2",
        },
        p2: {
            fontFamily: "2",
            fontSize: 12,
            fontColor: "3",
        },
    },
    chalkboard: {
        fontFamily1: `"Shesek - Regular", "Architects Daughter - Regular"`,
        fontFamily2: `"Frank Ruhl Libre - Medium", "Architects Daughter - Regular"`,
        fontFamily3: `"Shesek - Regular", "Roboto Slab - Regular"`,
        color1: "#272727",
        color2: "#AFCBF4",
        color3: "#D8E7FA",
        h1: {
            fontFamily: "1",
            fontSize: 28,
            fontColor: "2",
        },
        h2: {
            fontFamily: "2",
            fontSize: 25,
            fontColor: "3",
        },
        p1: {
            fontFamily: "1",
            fontSize: 14,
            fontColor: "2",
        },
        p2: {
            fontFamily: "2",
            fontSize: 12,
            fontColor: "3",
        },
    },
    ["happy-brush"]: {
        fontFamily1: `"Frank Ruhl Libre - Medium", "Roboto Slab - Medium"`,
        fontFamily2: `"Shesek - Regular", "Caveat - Medium"`,
        fontFamily3: "", 
        color1: "#272727",
        color2: "#FF6639",
        color3: "#FFF3ED",
        h1: {
            fontFamily: "1",
            fontSize: 28,
            fontColor: "2",
        },
        h2: {
            fontFamily: "2",
            fontSize: 25,
            fontColor: "3",
        },
        p1: {
            fontFamily: "1",
            fontSize: 14,
            fontColor: "2",
        },
        p2: {
            fontFamily: "2",
            fontSize: 12,
            fontColor: "3",
        },
    },
};


function setRootVars() {
    const style = document.documentElement.style;

    Object.entries(THEMES)
    .forEach(([id, theme]) => {
            const {fontFamily1, fontFamily2, fontFamily3, color1, color2, color3, h1, h2, p1, p2} = theme;
            style.setProperty(`--theme-${id}-color-1`, color1);
            style.setProperty(`--theme-${id}-color-2`, color2);
            style.setProperty(`--theme-${id}-color-3`, color3);
            style.setProperty(`--theme-${id}-font-family-1`, fontFamily1);
            style.setProperty(`--theme-${id}-font-family-2`, fontFamily2);
            style.setProperty(`--theme-${id}-font-family-3`, fontFamily3);
            //h1
            style.setProperty(`--theme-${id}-h1-font-family`, `var(--theme-${id}-font-family-${h1.fontFamily})`);
            style.setProperty(`--theme-${id}-h1-font-size`, `${h1.fontSize}px`); 
            style.setProperty(`--theme-${id}-h1-color`, `var(--theme-${id}-color-${h1.fontColor})`);
            //h2
            style.setProperty(`--theme-${id}-h2-font-family`, `var(--theme-${id}-font-family-${h2.fontFamily})`);
            style.setProperty(`--theme-${id}-h2-font-size`, `${h2.fontSize}px`); 
            style.setProperty(`--theme-${id}-h2-color`, `var(--theme-${id}-color-${h2.fontColor})`);
            //p1
            style.setProperty(`--theme-${id}-p1-font-family`, `var(--theme-${id}-font-family-${p1.fontFamily})`);
            style.setProperty(`--theme-${id}-p1-font-size`, `${p1.fontSize}px`); 
            style.setProperty(`--theme-${id}-p1-color`, `var(--theme-${id}-color-${p1.fontColor})`);
            //p2
            style.setProperty(`--theme-${id}-p2-font-family`, `var(--theme-${id}-font-family-${p2.fontFamily})`);
            style.setProperty(`--theme-${id}-p2-font-size`, `${p2.fontSize}px`); 
            style.setProperty(`--theme-${id}-p2-color`, `var(--theme-${id}-color-${p2.fontColor})`);
        });
}

setRootVars();
