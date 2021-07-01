import { LitElement, html, customElement, query, property, PropertyValues } from 'lit-element';
import React, { useMemo } from 'react';
import ReactDOM from 'react-dom';
import { BaseSelection, Descendant, Transforms } from 'slate';
import { Align, Color, ControllerState, defaultState, ElementType, Font, FontSize, Weight } from './wysiwyg-types';
import { EditorBackbone } from './slate-wysiwyg-react/EditorBackbone';
import { EditorComponent } from './slate-wysiwyg-react/EditorComponent';
import { baseStyles } from './styles';
import { ThemeKind, THEMES, TextEditor as TextEditorTheme} from '@elements/_themes/themes';
import { getThemeVars } from "./wysiwyg-theme";

@customElement("wysiwyg-base")
export class _ extends LitElement {
    componentRef?: EditorComponent;

    @property()
    theme: ThemeKind = "chalkboard";

    static get styles() {
        return baseStyles;
    }

    setValue<K extends keyof ControllerState>(key: K, value: ControllerState[K]) {
        const defaultValue = this.getDefault(key);
        let finalValue = key !== "element" && value === defaultValue ? undefined : value;
        this.backbone.setValue(key, finalValue);
    }

    private _font = this.getDefault('font');
    public set font(v: Font) {
        this.reFocus();
        this.setValue("font", v);
        this._font = v;
    }

    private _weight = this.getDefault('weight');
    public set weight(v: Weight) {
        this.reFocus();
        this.setValue("weight", v);
        this._weight = v;
    }

    private _color = this.getDefault('color');
    public set color(v: Color | undefined) {
        this.reFocus();
        this.setValue("color", v);
        this._color = v;
    }

    private _highlightColor = this.getDefault('highlightColor');
    public set highlightColor(v: Color | undefined) {
        this.reFocus();
        this.setValue("highlightColor", v);
        this._highlightColor = v;
    }

    private _indentCount = this.getDefault('indentCount');
    public set indentCount(v: number) {
        this.setValue("indentCount", v);
        this._indentCount = v;
    }

    private _element = this.getDefault('element');
    public set element(v: ElementType) {
        this.setValue("element", v);
        this._element = v;
    }

    private _fontSize = this.getDefault('fontSize');
    public set fontSize(v: number) {
        this.setValue("fontSize", v);
        this._fontSize = v;
    }

    private _italic = this.getDefault('italic');
    public set italic(v: boolean) {
        this.setValue("italic", v);
        this._italic = v;
    }

    private _underline = this.getDefault('underline');
    public set underline(v: boolean) {
        this.setValue("underline", v);
        this._underline = v;
    }

    private _align = this.getDefault('align');
    public set align(v: Align) {
        this.setValue("align", v);
        this._align = v;
    }

    updated(changedProperties: PropertyValues) {
        if (changedProperties.has('theme')) {
            this.onThemeChange();
        }
    }

    private onThemeChange() {
        getThemeVars(this.theme).forEach(([key, value]) => {
            this.style.setProperty(key, value);
        });
    }

    private getDefault<K extends keyof ControllerState>(key: K): ControllerState[K] {
        const elementType = this._element || this.elementDefault || defaultState.element;
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


    @query("#editorRoot")
    editorRoot!: HTMLElement;

    elementDefault?: ElementType;

    private get baseValue(): Descendant[] {
        let v = [
            {
                children: [{
                    text: ''
                }],
            },
        ] as any;

        if(this.elementDefault) v[0].element = this.elementDefault;

        return v;
    }

    private value: Descendant[] = this.baseValue;

    public resetValue() {
        this.value = JSON.parse(JSON.stringify(this.baseValue));
        this.componentRef?.setValue(this.value);
    }

    public set valueAsString(v: string) {
        if (!v) this.value = this.baseValue;
        else this.value = JSON.parse(v);
        this.componentRef?.setValue(this.value);
    }
    public get valueAsString(): string {
        return JSON.stringify(this.value);
    }

    private backbone = new EditorBackbone;

    private controlsChange<K extends keyof ControllerState>(key: K, value: ControllerState[K]) {
        if (value === undefined) value = null as any; // serde can't handle undefined only null
        this.dispatchEvent(new CustomEvent("wysiwyg-controls-change", {
            detail: {
                [key]: value
            }
        }));
    }

    private change(value: Descendant[]) {
        this.checkForControlsChange();
        this.checkForValueChangeChange(value);
    }

    private checkForValueChangeChange(newValue: Descendant[]) {
        const valueAsString = this.valueAsString;
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
        const leafFontSize = leaf?.fontSize || this.getDefault('fontSize');
        if(this._fontSize != leafFontSize) {
            this._fontSize = leafFontSize;
            this.controlsChange("fontSize", leafFontSize);
        }
        const leafItalic = leaf?.italic || this.getDefault('italic');
        if(this._italic != leafItalic) {
            this._italic = leafItalic;
            this.controlsChange("italic", leafItalic);
        }
        const leafUnderline = leaf?.underline || this.getDefault('underline');
        if(this._underline != leafUnderline) {
            this._underline = leafUnderline;
            this.controlsChange("underline", leafUnderline);
        }
        const leafWeight = leaf?.weight || this.getDefault('weight');
        if(this._weight != leafWeight) {
            this._weight = leafWeight;
            this.controlsChange("weight", leafWeight);
        }
        const leafFont = leaf?.font || this.getDefault('font');
        if(this._font != leafFont) {
            this._font = leafFont;
            this.controlsChange("font", leafFont);
        }
        const leafColor = leaf?.color || this.getDefault('color');
        if(this._color != leafColor) {
            this._color = leafColor;
            this.controlsChange("color", leafColor);
        }
        const leafHighlightColor = leaf?.highlightColor || this.getDefault('highlightColor');
        if(this._highlightColor != leafHighlightColor) {
            this._highlightColor = leafHighlightColor;
            this.controlsChange("highlightColor", leafHighlightColor);
        }

        const element = this.backbone.getSelectedElement();
        const elementAlign = element?.align || this.getDefault('align');
        if(this._align != elementAlign) {
            this._align = elementAlign;
            this.controlsChange("align", elementAlign);
        }
        const elementIndentCount = element?.indentCount || this.getDefault('indentCount');
        if(this._indentCount != elementIndentCount) {
            this._indentCount = elementIndentCount;
            this.controlsChange("indentCount", elementIndentCount);
        }
        const elementElement = element?.element || this.getDefault('element');
        if(this._element != elementElement) {
            this._element = elementElement;
            this.controlsChange("element", elementElement);
        }
    }

    public firstUpdated() {
        this.reactRender();
    }

    private _blurSelection?: BaseSelection;
    private onBlur(e: FocusEvent) {
        this._blurSelection = this.backbone.editor.selection;
        if(!this.closestPassShadow(e.relatedTarget as Node, "text-editor-controls")) {
            this.dispatchEvent(new Event("custom-blur"));
        }
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

    private reFocus() {
        if(this._blurSelection) {
            Transforms.select(this.backbone.editor, this._blurSelection);
        }
    }

    public selectAll() {
        const selection = window.getSelection();
        const range = document.createRange();
        range.selectNodeContents(this.shadowRoot!.querySelector("[contenteditable=true]")!);
        selection!.removeAllRanges();
        selection!.addRange(range);
    }

    private reactRender() {
        this.componentRef = ReactDOM.render(
            React.createElement(
                EditorComponent,
                {
                    backbone: this.backbone,
                    value: this.value,
                    onChange: (e) => this.change(e),
                    onBlur: (e: any) => this.onBlur(e),
                }
            ),
            this.editorRoot,
        );
    }

    public render() {
        return html`
            <div id="editorRoot"></div>
        `;
    }

}
