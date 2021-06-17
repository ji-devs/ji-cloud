import {ThemeKind} from "@elements/_themes/themes";
export * from "@elements/_themes/themes";

//TODO - import from config instead
export const ThemeKinds:Array<ThemeKind> = ["blank", "chalkboard", "happy-brush"];

export const ThemeControl = {
    control: {
            type: 'inline-radio',
            options: ThemeKinds 
    }
}
