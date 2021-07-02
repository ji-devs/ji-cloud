// this file needs to be in sync with frontend\apps\crates\components\src\text_editor\wysiwyg_types.rs


export type Color = string;
export type FontSize = number;
export type IndentCount = number;
export type Font = string;

export enum ElementType {
    H1 = "H1",
    H2 = "H2",
    P1 = "P1",
    P2 = "P2",
}
export type Weight = number;
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
    italic: boolean,
    underline: boolean,
    indentCount: IndentCount;
}

export const defaultState: ControllerState = {
    font: '"Roboto Slab - Regular"',
    element: ElementType.P1,
    weight: 400,
    align: Align.Left,
    fontSize: 16,

    // keep here even undefined for Object.keys
    color: undefined,
    highlightColor: undefined,
    italic: false,
    underline: false,
    indentCount: 0,
}


export function getKeyType<K extends keyof ControllerState>(key: K): 'leaf' | 'element' {
    if(key === "align" || key === "indentCount")
        return 'element';
    else
        return 'leaf';
}
