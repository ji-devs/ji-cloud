import { LitElement, html, css, customElement, internalProperty, property, PropertyValues } from 'lit-element';
import { CustomElement, CustomText } from './slate-wysiwyg-react/EditorBackbone';
import { StyleInfo, styleMap } from 'lit-html/directives/style-map';
import { baseStyles, getElementStyles, getLeafStyles } from './styles';
import { ThemeKind } from '@elements/_themes/themes';
import { getThemeVars } from './wysiwyg-theme';


@customElement("wysiwyg-output-renderer")
export class _ extends LitElement {

    static get styles() {
        return [baseStyles, css`
            :host {
                display: inline-block;
                /* TO FIX:
                    white-space: pre-wrap;
                */
                overflow-wrap: break-word;
            }
        `];
    }

    @internalProperty()
    private value: CustomElement[] = [];

    @property()
    private theme: ThemeKind = "chalkboard";

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

    public set valueAsString(v: string) {
        if (!v) this.value = [];
        else this.value = JSON.parse(v);
    }

    private renderElement(element: CustomElement) {
        const styles = getElementStyles(element) as StyleInfo;
        return html`
            <p style=${styleMap(styles)}>
                ${ element.children.map(leaf => {
                    return this.renderLeaf(leaf);
                }) }
            </p>
        `;
    }

    private renderLeaf(leaf: CustomText) {
        const styles = getLeafStyles(leaf) as StyleInfo;
        return html`<span style=${styleMap(styles)} type="${leaf.element}">
            ${ leaf.text === "" ? html`<br>` : leaf.text }
        </span>`;
    }

    public render() {
        return html`
            ${ this.value.map(element => {
                return this.renderElement(element);
            }) }
        `;
    }

}
