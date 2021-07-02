import { LitElement, html, customElement, query, property, PropertyValues, css } from 'lit-element';
import React, { useMemo } from 'react';
import ReactDOM from 'react-dom';
import { BaseSelection, Descendant, Point, Transforms } from 'slate';
import { ControllerState, controlNameList, defaultState, ElementType, getKeyType, WysiwygValue } from './wysiwyg-types';
import { EditorElement, EditorText, EditorBackbone } from './slate-wysiwyg-react/EditorBackbone';
import { EditorComponent } from './slate-wysiwyg-react/EditorComponent';
import { baseStyles } from './styles';
import { ThemeKind, THEMES, TextEditor as TextEditorTheme} from '@elements/_themes/themes';
import { getThemeVars } from "./wysiwyg-theme";

@customElement("wysiwyg-base")
export class _ extends LitElement {
    static get styles() {
        return [
            baseStyles,
            css`
                ::selection {
                    background-color: #00000020;
                }
            `,
        ];
    }

    @property()
    public theme: ThemeKind = "chalkboard";

    public elementDefault?: ElementType;

    @query("#editorRoot")
    private editorRoot!: HTMLElement;

    private componentRef?: EditorComponent;

    private backbone = new EditorBackbone;

    private controllerState: ControllerState = this.getDefaultState();

    private _blurSelection?: BaseSelection;

    private createValue(text: string = ""): WysiwygValue {
        let textNode: EditorText = {
            text
        };

        if(this.elementDefault) textNode.element = this.elementDefault;

        let v: WysiwygValue = {
            version: "0.1.0",
            content: [
                {
                    children: [textNode],
                },
            ],
        };

        return v;
    }

    private value: WysiwygValue = this.createValue();

    public set valueAsString(v: string) {
        if (!v) this.value = this.createValue();
        else this.value = JSON.parse(v);
        this.componentRef?.setValue(this.value.content);
    }
    public get valueAsString(): string {
        return JSON.stringify(this.value);
    }

    firstUpdated() {
        this.reactRender();
    }

    updated(changedProperties: PropertyValues) {
        if (changedProperties.has('theme')) {
            this.onThemeChange();
        }
    }

    createRenderRoot() {
        // hebrew keyboard only works when delegatesFocus is true
        return this.attachShadow({ mode: 'open', delegatesFocus: true });
    }

    render() {
        return html`
            <div id="editorRoot"></div>
        `;
    }

    public setTextAtSelection(text: string) {
        const currentSelection = this.backbone.editor.selection;
        if(currentSelection) {
            Transforms.insertText(this.backbone.editor, text, {
                at: currentSelection,
            });
        }
    }

    // if text is selected delete it otherwise delete the last character just like backspace
    public triggerBackspace() {
        const currentSelection = this.backbone.editor.selection;
        if(currentSelection) {
            // check if text is actually selected or it's just a cursor
            const isTextSelection = !Point.equals(currentSelection.anchor, currentSelection.focus);
            if(isTextSelection) {
                Transforms.delete(this.backbone.editor, {
                    at: currentSelection,
                });
            } else {
                this.backbone.editor.deleteBackward('character');
            }
        }
    }

    public selectAll() {
        const selection = window.getSelection()!;
        const range = document.createRange();
        range.selectNodeContents(this.shadowRoot!.querySelector("[contenteditable=true]")!);
        selection.removeAllRanges();
        selection.addRange(range);
    }

    public setControlValue<K extends keyof ControllerState>(key: K, value: ControllerState[K]) {
        this.reFocus();

        const defaultValue = this.getDefault(key);
        let finalValue = key !== "element" && value === defaultValue ? undefined : value;
        this.backbone.setValue(key, finalValue);
        this.controllerState[key] = value;

        if(key === "element") {
            // setting element resets all other values
            for (const key of controlNameList) {
                if(key === "element") continue;
                this.backbone.setValue(key as any, undefined);
            }
        }
    }

    public clearValue() {
        this.value = JSON.parse(JSON.stringify(this.createValue()));
        this.componentRef?.setValue(this.value.content);
    }

    private onThemeChange() {
        getThemeVars(this.theme).forEach(([key, value]) => {
            this.style.setProperty(key, value);
        });
    }


    private getDefaultState(): ControllerState {
        const entries = controlNameList.map(key => [key, this.getDefault(key)]);
        return Object.fromEntries(entries);
    }

    private getDefault<K extends keyof ControllerState>(key: K): ControllerState[K] {
        const elementType = this.controllerState?.element || this.elementDefault || defaultState.element;
        const elementName:keyof TextEditorTheme = elementType.toLowerCase() as any;

        const themeInfo = THEMES[this.theme];

        switch (key) {
            case "color":
                let color = (themeInfo as any)["color" + themeInfo.textEditor[elementName].fontColor];
                return `#${color[0].toString(16)}${color[1].toString(16)}${color[2].toString(16)}` as any;
            case "font":
                return (themeInfo as any)["fontFamily" + themeInfo.textEditor[elementName].fontFamily];
            case "fontSize":
                // for some reason I need any
                return themeInfo.textEditor[elementName].fontSize as any;
            default:
                return defaultState[key];
        }
    }

    private triggerControlsChangeEvent<K extends keyof ControllerState>(key: K, value: ControllerState[K]) {
        if (value === undefined) value = null as any; // serde can't handle undefined only null
        this.dispatchEvent(new CustomEvent("wysiwyg-controls-change", {
            detail: {
                [key]: value
            }
        }));
    }

    private onChange(value: Descendant[]) {
        this.checkForControlsChange();
        this.checkForValueChangeChange(value as EditorElement[]);
    }

    private checkForValueChangeChange(newContent: EditorElement[]) {
        const valueAsString = this.valueAsString;
        let newValue = JSON.parse(JSON.stringify(this.value)) as WysiwygValue;
        newValue.content = newContent;
        const newValueAsString = JSON.stringify(newValue);
        if(valueAsString !== newValueAsString) {
            this.dispatchEvent(new CustomEvent("custom-change", {
                detail: {
                    value: newValueAsString
                }
            }));
        }
        this.value = newValue;
    }

    private checkForControlsChange() {
        const leaf = this.backbone.getSelectedLeaf();
        const element = this.backbone.getSelectedElement();

        for (const key of controlNameList) {
            let node: any = getKeyType(key) === 'element' ? element
                : leaf;

            const controlValue = node?.[key] || this.getDefault(key);
            if(this.controllerState[key] != controlValue) {
                (this.controllerState as any)[key] = controlValue;
                this.triggerControlsChangeEvent(key, controlValue);
            }
        }
    }

    private onBlur(e: FocusEvent) {
        this._blurSelection = this.backbone.editor.selection;
        if(!this.closestPassShadow(e.relatedTarget as Node, "text-editor-controls")) {
            this.dispatchEvent(new Event("custom-blur"));
        }
    }

    private reFocus() {
        if(this._blurSelection) {
            (this.shadowRoot!.querySelector("[contenteditable=true]") as HTMLElement).focus();

            Transforms.select(this.backbone.editor, this._blurSelection);
        }
    }

    private reactRender() {
        this.componentRef = ReactDOM.render(
            React.createElement(
                EditorComponent,
                {
                    backbone: this.backbone,
                    value: this.value.content,
                    onChange: (e) => this.onChange(e),
                    onBlur: (e: any) => this.onBlur(e),
                }
            ),
            this.editorRoot,
        );
    }

    private closestPassShadow(node: Node | null, selector: string) : HTMLElement | null {
        if (!node) {
            return null;
        }
        if (node instanceof ShadowRoot) {
            return this.closestPassShadow(node.host, selector);
        }
        if (node instanceof HTMLElement) {
            if (node.matches(selector)) {
                return node;
            } else {
                return this.closestPassShadow(node.parentNode, selector);
            }
        }
        return this.closestPassShadow(node.parentNode, selector);
    }
}
