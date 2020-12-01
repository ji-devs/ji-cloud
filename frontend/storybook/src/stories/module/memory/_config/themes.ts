import THEMES_JSON from "@frontend-config/module/memory/themes.json";

export interface Theme {
    id: string,
    content: string,
    label: string
}

export const getAllThemes = ():Theme[] => {
    return THEMES_JSON.themes;
}
export const getThemeById = (theme_id:string):Theme => {
    return getAllThemes().find(({id}) => id == theme_id) as Theme;
}

export const getThemeByIndex = (index:number):Theme => {
    return getAllThemes()[index] as Theme;
}