import { LitElement, html, customElement, internalProperty } from 'lit-element';
import { CustomElement, CustomText } from './slate-wysiwyg-react/EditorBackbone';
import { StyleInfo, styleMap } from 'lit-html/directives/style-map';
import { baseStyles, getElementStyles, getLeafStyles } from './styles';


@customElement("wysiwyg-output-renderer")
export class _ extends LitElement {

    static get styles() {
        return baseStyles;
    }

    @internalProperty()
    private value: CustomElement[] = [];

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
        return html`<span style=${styleMap(styles)}>
            ${ leaf.text }
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
