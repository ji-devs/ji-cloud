import {
    LitElement,
    html,
    css,
    customElement,
    property,
    PropertyValues,
    state,
} from "lit-element";
import {
    EditorElement,
    EditorText,
} from "./slate-wysiwyg-react/EditorBackbone";
import { StyleInfo, styleMap } from "lit-html/directives/style-map";
import {
    baseStyles,
    getElementStyles,
    getLeafStyles,
    getRootStyles,
} from "./styles";
import { ThemeId } from "@elements/_themes/themes";
import { getThemeVars } from "./wysiwyg-theme";
import { WysiwygValue } from "./wysiwyg-types";
import { ifDefined } from "lit-html/directives/if-defined";

function getDefaultValue(): WysiwygValue {
    return {
        version: "0.1.0",
        content: [],
    };
}

@customElement("wysiwyg-output-renderer")
export class _ extends LitElement {
    static get styles() {
        return [
            baseStyles,
            css`
                :host {
                    white-space: pre-wrap;
                    overflow-wrap: break-word;
                }
            `,
        ];
    }

    @state()
    private value: WysiwygValue = getDefaultValue();

    @property()
    private theme: ThemeId = "chalkboard";

    updated(changedProperties: PropertyValues) {
        if (changedProperties.has("theme")) {
            this.onThemeChange();
        }
    }

    private onThemeChange() {
        getThemeVars(this.theme).forEach(([key, value]) => {
            this.style.setProperty(key, value);
        });
    }

    public set valueAsString(v: string) {
        if (!v) this.value = getDefaultValue();
        else this.value = JSON.parse(v);
    }

    public get textValue() {
        const text = [];
        for (const element of this.value.content) {
            for (const leaf of element.children) {
                if (leaf.text) {
                    text.push(leaf.text);
                }
            }
        }

        return text.join("");
    }

    /// This will set the value in the first leaf which has text. All other elements and their children will be removed.
    public set textValue(value: string) {
        let editor_element: EditorElement | null = null;
        let editor_text: EditorText | null = null;

        for (const element of this.value.content) {
            for (const leaf of element.children) {
                if (leaf.text) {
                    editor_element = element;
                    editor_text = leaf;
                }
            }
        }

        if (editor_element && editor_text) {
            editor_text.text = value;
            editor_element.children = [editor_text];
            this.value.content = [editor_element];
            // LitElement needs a push to tell it that the state has actually been updated.
            this.requestUpdate();
        }
    }

    // keep the render functions in one line since we're using `white-space: pre-wrap` so every extra new-line or whitespace will be reflected on the output
    // prettier-ignore
    private renderElement(element: EditorElement) {
        const styles = getElementStyles(element) as StyleInfo;
        return html`<p style=${styleMap(styles)}>${element.children.map((leaf) => {
            return this.renderLeaf(leaf);
        })}</p>`;
    }

    // prettier-ignore
    private renderLeaf(leaf: EditorText) {
        const styles = getLeafStyles(leaf) as StyleInfo;
        return html`<span style=${styleMap(styles)} type="${ifDefined(leaf.element)}" dir="auto">${leaf.text === "" ? html`<br />` : leaf.text}</span>`;
    }

    // prettier-ignore
    public render() {
        return html`${getRootStyles(this.value)}${this.value.content.map((element) => {
            return this.renderElement(element);
        })}`;
    }
}
