import { LitElement, html, css, customElement, property, query } from 'lit-element';
import React, { useMemo } from 'react';
import ReactDOM from 'react-dom';

import { BaseSelection, Descendant, Transforms } from 'slate';
import { Align, Color, ControllerState, ElementType, Font, FontSize, getDefault, getKeyType, Weight } from './wysiwyg-types';
import { EditorBackbone } from './slate-wysiwyg-react/EditorBackbone';
import { EditorComponent } from './slate-wysiwyg-react/EditorComponent';

@customElement("wysiwyg-base")
export class _ extends LitElement {

    static get styles() {
        return [
            css`
                h1, h2, p {
                    margin: 0;
                    font-weight: normal;
                }
                h1 {
                    font-size: 34px;
                }
                h2 {
                    font-size: 23px;
                }
                p[p1] {
                    font-size: 16px;
                }
                p[p2] {
                    font-size: 14px;
                }
            `,
        ];
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
        console.log(v);
        
        this.backbone.setValue("color", v);
        this._color = v;
    }

    private _highlightColor = getDefault('highlightColor');
    public set highlightColor(v: Color | undefined) {
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

    private _bold = getDefault('bold');
    public set bold(v: boolean) {
        this.backbone.setValue("bold", v);
        this._bold = v;
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

    private static baseValue: any[] = [
        {
            element: ElementType.P1,
            children: [{ text: '' }],
        },
    ];

    private value: Descendant[] = _.baseValue;

    public set valueAsString(v: string) {
        if (!v) this.value = _.baseValue;
        else this.value = JSON.parse(v);
        this.reactRender();
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
        if(valueAsString !== JSON.stringify(newValue)) {
            this.dispatchEvent(new CustomEvent("custom-change", {
                detail: {
                    value: valueAsString
                }
            }));
        }
        this.value = newValue;
    }

    private __checkForControlsChange<K extends keyof ControllerState>(key: K) {
        const node = getKeyType(key) === "leaf" ? this.backbone.getSelectedLeaf() : this.backbone.getSelectedElement();
        const thisKey = (this as any)['_' + key];
        const _this = this as any; // doing this just to get type any

        const nodeValue = (node as any)?.[key] || getDefault(key);
        if(_this[thisKey] != nodeValue) {
            _this[thisKey] = nodeValue;
            this.controlsChange(key, nodeValue);
        }
    }

    private checkForControlsChange() {
        const leaf = this.backbone.getSelectedLeaf();
        const leafFontSize = leaf?.fontSize || getDefault('fontSize');
        if(this._fontSize != leafFontSize) {
            this._fontSize = leafFontSize;
            this.controlsChange("fontSize", leafFontSize);
        }
        const leafBold = leaf?.bold || getDefault('bold');
        if(this._bold != leafBold) {
            this._bold = leafBold;
            this.controlsChange("bold", leafBold);
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
    private onBlur() {
        this._blurSelection = this.backbone.editor.selection;
        console.log(this._blurSelection);
    }

    private reFocus() {
        if(this._blurSelection) {
            Transforms.select(this.backbone.editor, this._blurSelection);
        }
    }

    private reactRender() {
        ReactDOM.render(
            React.createElement(
                EditorComponent,
                {
                    backbone: this.backbone,
                    value: this.value,
                    onChange: (e) => this.change(e),
                    onBlur: () => this.onBlur(),
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
