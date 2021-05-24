import { LitElement, html, customElement, query } from 'lit-element';
import React, { useMemo } from 'react';
import ReactDOM from 'react-dom';

import { BaseSelection, Descendant, Transforms } from 'slate';
import { Align, Color, ControllerState, ElementType, Font, FontSize, getDefault, Weight } from './wysiwyg-types';
import { EditorBackbone } from './slate-wysiwyg-react/EditorBackbone';
import { EditorComponent } from './slate-wysiwyg-react/EditorComponent';
import { baseStyles } from './styles';

@customElement("wysiwyg-base")
export class _ extends LitElement {
    componentRef?: EditorComponent;

    static get styles() {
        return baseStyles;
    }

    private _font = getDefault('font');
    public set font(v: Font) {
        this.reFocus();
        this.backbone.setValue("font", v);
        this._font = v;
    }

    private _weight = getDefault('weight');
    public set weight(v: Weight) {
        this.reFocus();
        this.backbone.setValue("weight", v);
        this._weight = v;
    }

    private _color = getDefault('color');
    public set color(v: Color | undefined) {
        this.reFocus();
        this.backbone.setValue("color", v);
        this._color = v;
    }

    private _highlightColor = getDefault('highlightColor');
    public set highlightColor(v: Color | undefined) {
        this.reFocus();
        this.backbone.setValue("highlightColor", v);
        this._highlightColor = v;
    }

    private _indentCount = getDefault('indentCount');
    public set indentCount(v: number) {
        this.backbone.setValue("indentCount", v);
        this._indentCount = v;
    }

    private _element = getDefault('element');
    public set element(v: ElementType) {
        this.backbone.setValue("element", v);
        this._element = v;
    }

    private _fontSize = getDefault('fontSize');
    public set fontSize(v: number) {
        this.backbone.setValue("fontSize", v);
        this._fontSize = v;
    }

    private _italic = getDefault('italic');
    public set italic(v: boolean) {
        this.backbone.setValue("italic", v);
        this._italic = v;
    }

    private _underline = getDefault('underline');
    public set underline(v: boolean) {
        this.backbone.setValue("underline", v);
        this._underline = v;
    }

    private _align = getDefault('align');
    public set align(v: Align) {
        this.backbone.setValue("align", v);
        this._align = v;
    }


    @query("#editorRoot")
    editorRoot!: HTMLElement;

    elementDefault?: ElementType;
    fontDefault?: Font;
    fontSizeDefault?: FontSize;
    colorDefault?: Color;

    private get baseValue(): Descendant[] {
        let v = [
            {
                children: [{
                    text: ''
                }],
            },
        ] as any;

        if(this.elementDefault) v[0].element = this.elementDefault;
        if(this.fontDefault) v[0].children[0].font = this.fontDefault;
        if(this.fontSizeDefault) v[0].children[0].fontSize = this.fontSizeDefault;
        if(this.colorDefault) v[0].children[0].color = this.colorDefault;

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
        const leafFontSize = leaf?.fontSize || getDefault('fontSize');
        if(this._fontSize != leafFontSize) {
            this._fontSize = leafFontSize;
            this.controlsChange("fontSize", leafFontSize);
        }
        const leafItalic = leaf?.italic || getDefault('italic');
        if(this._italic != leafItalic) {
            this._italic = leafItalic;
            this.controlsChange("italic", leafItalic);
        }
        const leafUnderline = leaf?.underline || getDefault('underline');
        if(this._underline != leafUnderline) {
            this._underline = leafUnderline;
            this.controlsChange("underline", leafUnderline);
        }
        const leafWeight = leaf?.weight || getDefault('weight');
        if(this._weight != leafWeight) {
            this._weight = leafWeight;
            this.controlsChange("weight", leafWeight);
        }
        const leafFont = leaf?.font || getDefault('font');
        if(this._font != leafFont) {
            this._font = leafFont;
            this.controlsChange("font", leafFont);
        }
        const leafColor = leaf?.color || getDefault('color');
        if(this._color != leafColor) {
            this._color = leafColor;
            this.controlsChange("color", leafColor);
        }
        const leafHighlightColor = leaf?.highlightColor || getDefault('highlightColor');
        if(this._highlightColor != leafHighlightColor) {
            this._highlightColor = leafHighlightColor;
            this.controlsChange("highlightColor", leafHighlightColor);
        }

        const element = this.backbone.getSelectedElement();
        const elementAlign = element?.align || getDefault('align');
        if(this._align != elementAlign) {
            this._align = elementAlign;
            this.controlsChange("align", elementAlign);
        }
        const elementIndentCount = element?.indentCount || getDefault('indentCount');
        if(this._indentCount != elementIndentCount) {
            this._indentCount = elementIndentCount;
            this.controlsChange("indentCount", elementIndentCount);
        }
        const elementElement = element?.element || getDefault('element');
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
