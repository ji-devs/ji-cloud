/*** Configure Mappings Here ***/

/*
    The basic idea is we define the values at the top level (fontFamilyN, colorN)
    and then we _map_ to those values in various use cases (text editor, card color, etc.)

    the exception to this is font sizes which are straight, unmapped values
    font sizes are numbers - do not specify px vs. rem since it differs in the use case
*/
import THEMES_JSON from "../../../config/themes.json";
import {hexStringToNumber, hexNumberToRgb} from "@utils/hex";

export type ThemeId = keyof typeof THEMES_JSON;

export const THEMES = THEMES_JSON as Record<ThemeId, Theme>;
//These are just for TS help, real ThemeId is defined in the Rust shared crate
//There is probably a way to get it from Object.keys(THEMES) ?

/******** Nothing to configure below this line **********/


//Typescript definitions
export interface Theme {
    id: string,
    label: {
        en: string
    },
    fontFamilies: Array<string>,
    colors: Array<string>,
    textEditor: TextEditor,
    cards: Cards
}

// aliases just for clarity
type FontFamilyMapping = number;
type ColorMapping = number; 

export interface TextEditor {
    h1: TextEditorVariant,
    h2: TextEditorVariant,
    p1: TextEditorVariant,
    p2: TextEditorVariant,
    fontList: Array<string>,
}
export interface TextEditorVariant {
    fontFamily: FontFamilyMapping,
    fontSize: number //direct value
    fontColor: ColorMapping
}

export interface Cards {
    fontColor: ColorMapping,
    fillColor: ColorMapping,
    borderColor: ColorMapping,
    fontFamily: FontFamilyMapping,
    fontFamilyLetteringLeft: FontFamilyMapping,
    fontFamilyLetteringRight: FontFamilyMapping,
}

function setRootVars() {
    const style = document.documentElement.style;

    Object.entries(THEMES as Record<ThemeId, Theme>)
        .forEach(([id, theme]) => {
            const rgb = (mapping:ColorMapping, kind: "color" | "var"):string => {
                const hexString = theme.colors[mapping];
                const hexNumber = hexStringToNumber(hexString);
                const [r, g, b] = hexNumberToRgb(hexNumber);
                if(kind === "var") {
                    return `${r}, ${g}, ${b}`
                } else {
                    return `rgba(${r}, ${g}, ${b}, 1.0)`
                } 
            }
            
            const fontFamily = (mapping:FontFamilyMapping):string => {
                return theme.fontFamilies[mapping];
            }

            //text editor
            (() => {
                Object.entries(theme.textEditor)
                    .filter(key => ["h1", "h2", "p1", "p2"].indexOf(key as any) !== -1)
                    .forEach(([key, value]) => {
                        console.log(key, value);

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
