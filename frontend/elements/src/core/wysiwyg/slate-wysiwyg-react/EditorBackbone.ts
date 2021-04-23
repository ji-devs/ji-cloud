import { Editor, Transforms, Text, createEditor, BaseEditor, NodeEntry, Node, Element as SlateElement } from "slate";
import { ReactEditor, withReact } from "slate-react";
import { ControllerState, Align, Color, ElementType, Font, FontSize, IndentCount, Weight, getDefault, getKeyType } from "../wysiwyg-types";

export type CustomElement = {
    children: CustomText[];
    element: ElementType;
    align: Align,
    indentCount: IndentCount,
}
export type CustomText = {
    text: string;
    bold: boolean;
    underline: boolean;
    italic: boolean;
    color: Color;
    highlightColor: Color;
    fontSize: FontSize;
    weight: Weight;
    font: Font;
}

declare module 'slate' {
    interface CustomTypes {
        Editor: BaseEditor & ReactEditor;
        Element: CustomElement;
        Text: CustomText;
    }
}

type BooleanControlKey = Extract<{ [key in keyof ControllerState]: ControllerState[key] extends boolean ? key : never }[keyof ControllerState], string>;

export class EditorBackbone {

    private _editor = withReact(createEditor());
    public get editor() {
        return this._editor;
    }

    public readonly keyMaps = new Map([
        ['u', () => this.toggleMark('underline')],
        ['b', () => this.toggleMark('bold')],
        ['i', () => this.toggleMark('italic')],
    ]);

    public isMarkActive(key: BooleanControlKey): boolean {
        let iterators = Editor.nodes(this.editor, {
            match: (n: any) => n[key] === true,
            universal: true,
        });
        return !iterators.next().done;
    }

    toggleMark(key: BooleanControlKey) {
        const isActive = this.getValue(key);
        Transforms.setNodes(
            this._editor,
            { [key]: isActive ? null : true },
            { match: n => Text.isText(n), split: true }
        )
    }

    getValue<K extends keyof ControllerState>(key: K): ControllerState[K] | undefined {
        const keyType = getKeyType(key);
        if(keyType === 'element') {
            return (this.getSelectedElement() as any)?.[key];
        } else {
            return (this.getSelectedLeaf() as any)?.[key];
        }
    }

    getSelectedLeaf(): CustomText | undefined {
        if(!this.editor.selection) return;
        const [fistSelectedElement] = Editor.node(this.editor, this.editor.selection);
        return fistSelectedElement as CustomText;
    }

    getSelectedElement() : CustomElement | undefined {
        if(!this.editor.selection) return;
        let iterator = Editor.nodes(this.editor, {
            at: this.editor.selection,
            match: n => SlateElement.isElement(n),
        });
        return (iterator.next().value as any)[0] as CustomElement | undefined;
    }

    setValue<K extends keyof ControllerState>(key: K, value: ControllerState[K]) {
        const defaultValue = getDefault(key);
        let finalValue = value === defaultValue ? undefined : value;
        const keyType = getKeyType(key);
        if(keyType === 'element') {
            Transforms.setNodes(
                this._editor,
                { [key]: finalValue },
                { match: n => Editor.isBlock(this._editor, n) }
            );
        } else {
            Transforms.setNodes(
                this._editor,
                {[key]: finalValue},
                { match: n => Text.isText(n), split: true }
            );
        }
    }

}
