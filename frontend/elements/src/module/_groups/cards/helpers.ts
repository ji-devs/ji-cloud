import { styleMap } from 'lit-html/directives/style-map';
import {ThemeKind} from "@elements/_themes/themes";

export const cardBackPath = (theme:ThemeKind):string => {
    return `theme/module/_groups/cards/${theme}/card-back.png`;
}

export type Mode = 
	"duplicate"
        | "words-images"
        | "begins-with"
        | "lettering"
        | "riddles"
        | "opposites"
        | "synonymns"
        | "translate"

export const getFrontStyle = (theme:ThemeKind, mode: Mode) => {
      return styleMap({
          "--color": `var(--theme-${theme}-cards-color)`,
          "--font-family": mode === "lettering" 
            ? `var(--theme-${theme}-cards-font-family-lettering)`
            : `var(--theme-${theme}-cards-font-family)`,
          borderColor: `var(--theme-${theme}-cards-border-color)`,
          backgroundColor: `var(--theme-${theme}-cards-fill-color)`,
      });
}