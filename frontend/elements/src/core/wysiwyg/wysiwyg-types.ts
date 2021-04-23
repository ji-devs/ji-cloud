// this file needs to be in sync with frontend\apps\crates\components\src\text_editor\wysiwyg_types.rs


export type Color = string;
export type FontSize = number;
export type IndentCount = number;

export type Font = "arial" | "roboto" | "open-sans";

export enum ElementType {
    H1 = "H1",
    H2 = "H2",
    P1 = "P1",
    P2 = "P2",
}
export enum Weight {
    Bolder = "Bolder",
    Bold = "Bold",
    Normal = "Normal",
    Lighter = "Lighter",
}
export enum Align {
    Left = "Left",
    Center = "Center",
    Right = "Right",
}

export interface ControllerState {
    font: Font,
    element: ElementType,
    weight: Weight,
    align: Align,
    fontSize: FontSize,
    color?: Color,
    highlightColor?: Color,
    bold: boolean,
    italic: boolean,
    underline: boolean,
    indentCount: IndentCount;
}

export const defaultState: ControllerState = {
    font: 'arial',
    element: ElementType.P1,
    weight: Weight.Normal,
    align: Align.Left,
    fontSize: 10,
    bold: false,
    italic: false,
    underline: false,
    indentCount: 0,
}

export function getDefault<K extends keyof ControllerState>(key: K): ControllerState[K] {
    return defaultState[key];
}

export function getKeyType<K extends keyof ControllerState>(key: K): 'leaf' | 'element' {
    if(key === "align" || key === "indentCount" || key === "element")
        return 'element';
    else
        return 'leaf';
}
