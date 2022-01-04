import { styleMap } from "lit-html/directives/style-map";
import { ThemeId } from "@elements/_themes/themes";

export const cardBackIconPath = (theme: ThemeId): string => {
    return `theme/${theme}/card-back-icon.png`;
};
export const cardBackFullPath = (theme: ThemeId): string => {
    return `theme/${theme}/card-back.png`;
};

//sets the style kind on the card itself
//doesn't affect inner content
//i.e. "none" will still use theme settings for font, but otherwise plain white card
export type StyleKind = "theme" | "none" | "dragging";

export type Effect = "positive" | "negative" | null;

export type Side = "left" | "right";

export type Mode =
    | "duplicate"
    | "words-images"
    | "begins-with"
    | "lettering"
    | "riddles"
    | "opposites"
    | "synonyms"
    | "translate";

export const getEmptyStyle = (
    theme: ThemeId,
    active: boolean,
    bgOpacity: number = 1.0
) => {
    return styleMap({
        "--color": `var(--theme-${theme}-cards-color)`,
        borderColor: `var(--theme-${theme}-cards-border-color)`,
        backgroundColor: active
            ? bgOpacity === 1.0
                ? `var(--theme-${theme}-cards-border-color)`
                : `rgb(var(--theme-${theme}-cards-border-color-var), ${bgOpacity})`
            : bgOpacity === 1.0
            ? `var(--theme-${theme}-cards-fill-color)`
            : `rgb(var(--theme-${theme}-cards-fill-color-var), ${bgOpacity})`,
    });
};

export const getContentStyle = (
    styleKind: StyleKind,
    theme: ThemeId,
    mode: Mode,
    side: Side,
    bgOpacity: number = 1.0
) => {
    return styleMap({
        "--color": `var(--theme-${theme}-cards-color)`,
        borderColor:
            styleKind === "dragging"
                ? "#1160fb"
                : styleKind === "none"
                ? "transparent"
                : `var(--theme-${theme}-cards-border-color)`,
        backgroundColor:
            styleKind === "dragging" || styleKind === "none"
                ? "white"
                : bgOpacity === 1.0
                ? `var(--theme-${theme}-cards-fill-color)`
                : `rgb(var(--theme-${theme}-cards-fill-color-var), ${bgOpacity})`,
        "--font-family":
            mode === "lettering"
                ? side === "left"
                    ? `var(--theme-${theme}-cards-font-family-lettering-left)`
                    : `var(--theme-${theme}-cards-font-family-lettering-right)`
                : `var(--theme-${theme}-cards-font-family)`,
    });
};
