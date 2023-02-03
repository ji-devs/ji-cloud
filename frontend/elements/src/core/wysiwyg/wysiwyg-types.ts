// this file needs to be in sync with frontend\apps\crates\components\src\text_editor\wysiwyg_types.rs

import { EditorElement } from "./slate-wysiwyg-react/EditorBackbone";

export type WysiwygValueVersion = "0.1.0";

export interface WysiwygValue {
    content: EditorElement[];
    boxColor?: Color;
    version: WysiwygValueVersion;
}

export type Color = string;
export type FontSize = number;
export type Font = string;

export enum Direction {
    LeftToRight = "LeftToRight",
    RightToLeft = "RightToLeft",
}

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
    font: Font;
    element: ElementType;
    weight: Weight;
    align: Align;
    fontSize: FontSize;
    color?: Color;
    highlightColor?: Color;
    boxColor?: Color;
    italic: boolean;
    underline: boolean;
}

export const defaultState: ControllerState = {
    font: '"Roboto Slab - Regular"',
    element: ElementType.H1,
    weight: 400,
    align: Align.Left,
    fontSize: 16,

    // keep here even undefined for Object.keys
    color: undefined,
    highlightColor: undefined,
    boxColor: undefined,
    italic: false,
    underline: false,
};

export type KeyLevel = "leaf" | "element" | "root";

export function getKeyLevel<K extends keyof ControllerState>(key: K): KeyLevel {
    // if (key === "align" || key === "indentCount") return "element";
    if (key === "align") return "element";
    if (key === "boxColor") return "root";
    else return "leaf";
}

export type ControlName = keyof ControllerState;

export const controlNameList = Object.keys(defaultState) as ControlName[];
