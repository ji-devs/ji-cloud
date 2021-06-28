/*** Configure Mappings Here ***/

/*
    The basic idea is we define the values at the top level (fontFamilyN, colorN)
    and then we _map_ to those values in various use cases (text editor, card color, etc.)

    the exception to this is font sizes which are straight, unmapped values
    font sizes are numbers - do not specify px vs. rem since it differs in the use case
*/

export const THEMES:Record<ThemeKind, Theme> = {
    blank: {
        fontFamily1: `"Arial", "Times New Roman"`,
        fontFamily2: `"Shesek, Cursive"`, 
        color1: [0xff, 0x23, 0x54],
        color2: [0xff, 0x23, 0x54],
        color3: [0xff, 0x23, 0x54],
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
        cards: getCardsThemeDefault()
    },
    chalkboard: {
        fontFamily1: `"Frank Ruhl Libre - Medium", "Roboto Slab - Medium"`,
        fontFamily2: `"Shesek - Regular", "Architects Daughter - Regular"`,
        color1: [0x27,0x27,0x27],
        color2: [0xAF,0xCB,0xF4],
        color3: [0xD8,0xE7,0xFA],
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
            ...getCardsThemeDefault(),
            fontFamilyLetteringLeft: 1,
            fontFamilyLetteringRight: 2,
        }
    },
    ["happy-brush"]: {
        fontFamily1: `"Frank Ruhl Libre - Medium", "Roboto Slab - Medium"`,
        fontFamily2: `"Shesek - Regular", "Caveat - Medium"`,
        color1: [0x27,0x27,0x27],
        color2: [0xFF,0x66,0x39],
        color3: [0xFF,0xF3,0xED],
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
            ...getCardsThemeDefault(),
            fontFamilyLetteringLeft: 1,
            fontFamilyLetteringRight: 2,
        }
    },
};

//convenience to set common card values
function getCardsThemeDefault():CardsTheme {
    return {
        fontColor: 1,
        borderColor: 2,
        fillColor: 3,
        fontFamily: 1,
        fontFamilyLetteringLeft: 1,
        fontFamilyLetteringRight: 1,
    }
}

export const STR_THEME_LABEL:Record<ThemeKind, string> = {
    "blank": "Blank",
    "chalkboard": "Chalkboard",
    "happy-brush": "Happy Brush"
}

/******** Nothing to configure below this line **********/

//These are just for TS help, real ThemeId is defined in the Rust shared crate
export type ThemeKind = "blank" | "chalkboard" | "happy-brush"

//Typescript definitions
export interface Theme {
    fontFamily1: string, //Regular
    fontFamily2: string, //Handwriting
    //Stored as separate RGB values
    //So that we can add opacity in CSS
    color1: RGB,
    color2: RGB,
    color3: RGB,
    //Text editor settings for h1-p2
    textEditor: TextEditor,
    //Card theme settings
    cards: CardsTheme
}

type RGB = [number, number, number];

type FontFamilyMapping = 1 | 2;
type ColorMapping = 1 | 2 | 3;

export interface TextEditor {
    h1: TextEditorVariant,
    h2: TextEditorVariant,
    p1: TextEditorVariant,
    p2: TextEditorVariant,
}


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
    fontFamilyLetteringLeft: FontFamilyMapping,
    fontFamilyLetteringRight: FontFamilyMapping,
}


export type FontFamily = {
    en: string,
    he: string,
}



function setRootVars() {
    const style = document.documentElement.style;


    Object.entries(THEMES)
    .forEach(([id, theme]) => {

            const rgb = (mapping:ColorMapping, kind: "color" | "var"):string => {
                const [r, g, b] = 
                    mapping === 1 ? theme.color1
                    : mapping == 2 ? theme.color2
                    : theme.color3;
                
                if(kind === "var") {
                    return `${r}, ${g}, ${b}`
                } else {
                    return `rgba(${r}, ${g}, ${b}, 1.0)`
                } 
            }
            
            const fontFamily = (mapping:FontFamilyMapping):string => {
                return mapping == 1 ? theme.fontFamily1 : theme.fontFamily2;
            }

            //text editor
            (() => {
                Object.entries(theme.textEditor)
                    .forEach(([key, value]) => {
                        style.setProperty(`--theme-${id}-${key}-font-family`, fontFamily(value.fontFamily));
                        style.setProperty(`--theme-${id}-${key}-font-size`, `${value.fontSize}px`); 
                        style.setProperty(`--theme-${id}-${key}-color`, rgb(value.fontColor, "color"));
                        style.setProperty(`--theme-${id}-${key}-color-var`, rgb(value.fontColor, "var"));
                    });

            })();

            //cards
            (() => {
                const value = theme.cards;
                style.setProperty(`--theme-${id}-cards-color`, rgb(value.fontColor, "color")); 
                style.setProperty(`--theme-${id}-cards-color-var`, rgb(value.fontColor, "var")); 
                style.setProperty(`--theme-${id}-cards-border-color`, rgb(value.borderColor, "color")); 
                style.setProperty(`--theme-${id}-cards-border-color-var`, rgb(value.borderColor, "var")); 
                style.setProperty(`--theme-${id}-cards-fill-color`, rgb(value.fillColor, "color")); 
                style.setProperty(`--theme-${id}-cards-fill-color-var`, rgb(value.fillColor, "var")); 
                style.setProperty(`--theme-${id}-cards-font-family`, fontFamily(value.fontFamily)); 
                style.setProperty(`--theme-${id}-cards-font-family-lettering-left`, fontFamily(value.fontFamilyLetteringLeft)); 
                style.setProperty(`--theme-${id}-cards-font-family-lettering-right`, fontFamily(value.fontFamilyLetteringRight)); 
            })();
        });
}

setRootVars();
