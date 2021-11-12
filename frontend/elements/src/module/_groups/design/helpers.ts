import { ThemeId } from "@elements/_themes/themes";

export const themeIconPath = (theme: ThemeId, hover: boolean): string => {
    return `theme/${theme}/icon${hover ? "-hover" : ""}.jpg`;
};
