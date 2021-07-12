import { Editor, Transforms, Text, createEditor, BaseEditor, NodeEntry, Node, Element as SlateElement } from "slate";
import { ReactEditor, withReact } from "slate-react";
import { ControllerState, Align, Color, ElementType, Font, FontSize, IndentCount, Weight, getKeyLevel } from "../wysiwyg-types";

export type EditorElement = {
    children: EditorText[];
    align?: Align,
    indentCount?: IndentCount,
}
export type EditorText = {
    text?: string;
    underline?: boolean;
    italic?: boolean;
    color?: Color;
    highlightColor?: Color;
    fontSize?: FontSize;
    weight?: Weight;
    font?: Font;
    element?: ElementType;
}

declare module 'slate' {
    interface CustomTypes {
        Editor: BaseEditor & ReactEditor;
        Element: EditorElement;
        Text: EditorText;
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
        // ['b', () => this.toggleMark('bold')],
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
        const keyType = getKeyLevel(key);
        if(keyType === 'element') {
            return (this.getSelectedElement() as any)?.[key];
        } else if(keyType === "leaf") {
            return (this.getSelectedLeaf() as any)?.[key];
        } else {
            throw new Error("Should not be handled on this level");
        }
    }

    getSelectedLeaf(): EditorText | undefined {
        if(!this.editor.selection) return;
        const iterator = Editor.nodes(this.editor, {
            at: this.editor.selection,
            match: n => Text.isText(n),
        });
        return (iterator.next().value as any)[0] as Text | undefined;
    }

    getSelectedElement() : EditorElement | undefined {
        if(!this.editor.selection) return;
        let iterator = Editor.nodes(this.editor, {
            at: this.editor.selection,
            match: n => SlateElement.isElement(n),
        });
        return (iterator.next().value as any)[0] as EditorElement | undefined;
    }

    setValue<K extends keyof ControllerState>(key: K, value: ControllerState[K] | undefined) {
        const keyType = getKeyLevel(key);
        if(keyType === 'element') {
            Transforms.setNodes(
                this._editor,
                { [key]: value },
                { match: n => Editor.isBlock(this._editor, n) }
            );
        } else if(keyType === "leaf") {
            Transforms.setNodes(
                this._editor,
                {[key]: value},
                { match: n => Text.isText(n), split: true }
            );
        } else {
            throw new Error("Should not be handled on this level");
        }
    }

}
