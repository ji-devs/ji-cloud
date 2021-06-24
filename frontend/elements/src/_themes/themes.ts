export interface Theme {
    fontFamily1: string, //Regular
    fontFamily2: string, //Handwriting
    color1: string,
    color2: string,
    color3: string,
    textEditor: TextEditor,
    cards: CardsTheme
}

export interface TextEditor {
    h1: TextEditorVariant,
    h2: TextEditorVariant,
    p1: TextEditorVariant,
    p2: TextEditorVariant,
}

type FontFamilyMapping = 1 | 2;
type ColorMapping = 1 | 2 | 3;
export interface TextEditorVariant {
    fontFamily: FontFamilyMapping,
    fontSize: number //direct value
    fontColor: ColorMapping
}

export interface CardsTheme {
    //all these are mappings
    fontColor: ColorMapping,
    fillColor: ColorMapping,
    borderColor: ColorMapping,
    fontFamily: FontFamilyMapping,
    fontFamilyLettering: FontFamilyMapping,
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

const cardsThemeDefault:CardsTheme = {
    fontColor: 1,
    borderColor: 2,
    fillColor: 3,
    fontFamily: 1,
    fontFamilyLettering: 1,
}
//TODO - import the config JSON via typescript / rollup
//to match themes.rs
export const THEMES:Record<ThemeKind, Theme> = {
    blank: {
        fontFamily1: "_sans",
        fontFamily2: "", 
        color1: "black",
        color2: "black",
        color3: "white",
        textEditor: {
            h1: {
                fontFamily: 1,
                fontSize: 28,
                fontColor: 2,
            },
            h2: {
                fontFamily: 2,
                fontSize: 25,
                fontColor: 3,
            },
            p1: {
                fontFamily: 1,
                fontSize: 14,
                fontColor: 2,
            },
            p2: {
                fontFamily: 2,
                fontSize: 12,
                fontColor: 3,
            },
        },
        cards: cardsThemeDefault
    },
    chalkboard: {
        fontFamily1: `"Frank Ruhl Libre - Medium", "Architects Daughter - Regular"`,
        fontFamily2: `"Shesek - Regular", "Architects Daughter - Regular"`,
        color1: "#272727",
        color2: "#AFCBF4",
        color3: "#D8E7FA",
        textEditor: {
            h1: {
                fontFamily: 1,
                fontSize: 28,
                fontColor: 2,
            },
            h2: {
                fontFamily: 2,
                fontSize: 25,
                fontColor: 3,
            },
            p1: {
                fontFamily: 1,
                fontSize: 14,
                fontColor: 2,
            },
            p2: {
                fontFamily: 2,
                fontSize: 12,
                fontColor: 3,
            },
        },
        cards: {
            ...cardsThemeDefault,
            fontFamilyLettering: 2,
        }
    },
    ["happy-brush"]: {
        fontFamily1: `"Frank Ruhl Libre - Medium", "Roboto Slab - Medium"`,
        fontFamily2: `"Shesek - Regular", "Caveat - Medium"`,
        color1: "#272727",
        color2: "#FF6639",
        color3: "#FFF3ED",
        textEditor: {
            h1: {
                fontFamily: 1,
                fontSize: 28,
                fontColor: 2,
            },
            h2: {
                fontFamily: 2,
                fontSize: 25,
                fontColor: 3,
            },
            p1: {
                fontFamily: 1,
                fontSize: 14,
                fontColor: 2,
            },
            p2: {
                fontFamily: 2,
                fontSize: 12,
                fontColor: 3,
            },
        },
        cards: {
            ...cardsThemeDefault,
            fontFamilyLettering: 2,
        }
    },
};


function setRootVars() {
    const style = document.documentElement.style;

    Object.entries(THEMES)
    .forEach(([id, theme]) => {
            const {fontFamily1, fontFamily2, color1, color2, color3, textEditor, cards} = theme;

            //Mapping values
            style.setProperty(`--theme-${id}-color-1`, color1);
            style.setProperty(`--theme-${id}-color-2`, color2);
            style.setProperty(`--theme-${id}-color-3`, color3);
            style.setProperty(`--theme-${id}-font-family-1`, fontFamily1);
            style.setProperty(`--theme-${id}-font-family-2`, fontFamily2);

            //text editor
            (() => {
                const {h1, h2, p1, p2} = textEditor;
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

            })();

            //cards
            (() => {
                const {fontColor, borderColor, fillColor, fontFamily, fontFamilyLettering} = cards;
                style.setProperty(`--theme-${id}-cards-color`, `var(--theme-${id}-color-${fontColor})`);
                style.setProperty(`--theme-${id}-cards-border-color`, `var(--theme-${id}-color-${borderColor})`);
                style.setProperty(`--theme-${id}-cards-fill-color`, `var(--theme-${id}-color-${fillColor})`);
                style.setProperty(`--theme-${id}-cards-font-family`, `var(--theme-${id}-font-family-${fontFamily})`);
                style.setProperty(`--theme-${id}-cards-font-family-lettering`, `var(--theme-${id}-font-family-${fontFamilyLettering})`);
            })();
        });
}

setRootVars();
