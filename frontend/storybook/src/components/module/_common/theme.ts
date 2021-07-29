import {ThemeId, THEMES} from "@elements/_themes/themes";
export * from "@elements/_themes/themes";

export const ThemeIds:Array<ThemeId> = Object.keys(THEMES) as Array<ThemeId>; 

export const ThemeControl = {
    control: {
            type: 'inline-radio',
            options: ThemeIds 
    }
}

