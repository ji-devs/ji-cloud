/*** Configure Mappings Here ***/

/*
    The basic idea is we define the values at the top level (fontFamilyN, colorN)
    and then we _map_ to those values in various use cases (text editor, card color, etc.)

    the exception to this is font sizes which are straight, unmapped values
    font sizes are numbers - do not specify px vs. rem since it differs in the use case
*/
import { mediaUi } from "@utils/path";
import THEMES_JSON from "../../../config/themes.json";
import FONTS_JSON from "../../../config/fonts.json";
import { hexStringToNumber, hexNumberToRgb, rgbToHsl } from "@utils/hex";

export type ThemeId = keyof typeof THEMES_JSON;

export const THEMES = THEMES_JSON as Record<ThemeId, Theme>;
export const FONTS = FONTS_JSON as Record<FontFamilyName, FontInfo>;
//These are just for TS help, real ThemeId is defined in the Rust shared crate
//There is probably a way to get it from Object.keys(THEMES) ?

/******** Nothing to configure below this line **********/

export interface FontInfo {
    file: string;
    format: string;
    range?: string;
}

export type FontFamilyName = string;

//Typescript definitions
export interface Theme {
    id: string;
    label: {
        en: string;
    };
    fontFamilies: Array<string>;
    colors: Array<string>;
    textEditor: TextEditor;
    cards: Cards;
}

// aliases just for clarity
type FontFamilyMapping = number;
type ColorMapping = number;

export interface TextEditor {
    h1: TextEditorVariant;
    h2: TextEditorVariant;
    p1: TextEditorVariant;
    p2: TextEditorVariant;
    fontList: Array<string>;
}
export interface TextEditorVariant {
    fontFamily: FontFamilyMapping;
    fontSize: number; //direct value
    fontColor: ColorMapping;
}

export interface Cards {
    fontColor: ColorMapping;
    fillColor: ColorMapping;
    borderColor: ColorMapping;
    fontFamily: FontFamilyMapping;
    fontFamilyLetteringLeft: FontFamilyMapping;
    fontFamilyLetteringRight: FontFamilyMapping;
}

function setRootVars() {
    const style = document.documentElement.style;

    Object.entries(THEMES as Record<ThemeId, Theme>).forEach(([id, theme]) => {
        const rgb = (mapping: ColorMapping, kind: "color" | "var"): string => {
            const hexString = theme.colors[mapping];
            const hexNumber = hexStringToNumber(hexString);
            const [r, g, b] = hexNumberToRgb(hexNumber);
            if (kind === "var") {
                return `${r}, ${g}, ${b}`;
            } else {
                return `rgba(${r}, ${g}, ${b}, 1.0)`;
            }
        };

        // Returns a lightened color as a HSL value
        const lighten = (mapping: ColorMapping, amount: number): string => {
            const hexString = theme.colors[mapping];
            const hexNumber = hexStringToNumber(hexString);
            const [r, g, b] = hexNumberToRgb(hexNumber);
            let [h, s, l] = rgbToHsl(r, g, b);

            // Decrease the lightness to darken the color by {amount}
            l = l + amount > 100 ? 100 : l + amount;
            return `${h}, ${s}%, ${l}%`;
        };

        const fontFamily = (mapping: FontFamilyMapping): string => {
            return theme.fontFamilies[mapping];
        };

        //text editor
        (() => {
            Object.entries(theme.textEditor)
                .filter(
                    ([key, _]) =>
                        ["h1", "h2", "p1", "p2"].indexOf(key as any) !== -1
                )
                .forEach(([key, value]) => {
                    style.setProperty(
                        `--theme-${id}-${key}-font-family`,
                        fontFamily(value.fontFamily)
                    );
                    style.setProperty(
                        `--theme-${id}-${key}-font-size`,
                        `${value.fontSize}rem`
                    );
                    style.setProperty(
                        `--theme-${id}-${key}-color`,
                        rgb(value.fontColor, "color")
                    );
                    style.setProperty(
                        `--theme-${id}-${key}-color-var`,
                        rgb(value.fontColor, "var")
                    );
                });
        })();

        //cards
        (() => {
            const value = theme.cards;
            style.setProperty(
                `--theme-${id}-cards-color`,
                rgb(value.fontColor, "color")
            );
            style.setProperty(
                `--theme-${id}-cards-color-var`,
                rgb(value.fontColor, "var")
            );
            style.setProperty(
                `--theme-${id}-cards-border-color`,
                rgb(value.borderColor, "color")
            );
            style.setProperty(
                `--theme-${id}-cards-border-color-var`,
                rgb(value.borderColor, "var")
            );
            style.setProperty(
                `--theme-${id}-cards-border-color-light-hsl`,
                lighten(value.borderColor, 25)
            );
            style.setProperty(
                `--theme-${id}-cards-fill-color`,
                rgb(value.fillColor, "color")
            );
            style.setProperty(
                `--theme-${id}-cards-fill-color-var`,
                rgb(value.fillColor, "var")
            );
            style.setProperty(
                `--theme-${id}-cards-font-family`,
                fontFamily(value.fontFamily)
            );
            style.setProperty(
                `--theme-${id}-cards-font-family-lettering-left`,
                fontFamily(value.fontFamilyLetteringLeft)
            );
            style.setProperty(
                `--theme-${id}-cards-font-family-lettering-right`,
                fontFamily(value.fontFamilyLetteringRight)
            );
        })();
    });
}

const fontsQueued: Set<FontFamilyName> = new Set();

//It's safe to call these multiple times and just await the promise
//Fonts will only be loaded once
export function loadFonts(fonts: Array<FontFamilyName>): Promise<void> {
    fonts
        .filter((name) => !fontsQueued.has(name))
        .forEach((name) => {
            fontsQueued.add(name);

            const { file, format, range } = FONTS[name];

            const descriptors = {} as any;
            if (range && range != "") {
                descriptors.unicodeRange = range;
            }

            const url = mediaUi(`fonts/${file}`);

            const face = new FontFace(
                name,
                `url(${url}) format('${format}')`,
                descriptors
            );

            //Disabling this will make it just lazy-load when needed?
            face.load();

            document.fonts.add(face);
        });

    return document.fonts.ready.then(() => console.log("fonts are ready!"));
}

export function loadAllFonts(): Promise<void> {
    return loadFonts(Object.keys(FONTS));
}

setRootVars();
