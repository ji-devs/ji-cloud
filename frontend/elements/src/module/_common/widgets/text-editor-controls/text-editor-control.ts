import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/button-collection/button-collection-item";
import "@elements/core/images/ui";

export type ControlType = 'align-center' | 'align-left' | 'align-right' | 'bold' | 'color' | 'dir-ltr' | 'dir-rtl' | 'h1' | 'h2' | 'italic' | 'marker-color' | 'p1' | 'p2' | 'underline';

@customElement("text-editor-control")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            `,
        ];
    }

    @property()
    type: ControlType = 'h1';

    render() {
        return html`
            <button-collection-item>
                <img-ui path="module/_common/widgets/sidebar/text-editor-controls/${ this.type }.svg"></img-ui>
            </button-collection-item>
        `;
    }
}
