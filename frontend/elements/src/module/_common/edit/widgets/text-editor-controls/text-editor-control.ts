import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/button-collection/button-collection-item";
import "@elements/core/images/ui";

export type ControlType = 'align-center' | 'align-left' | 'align-right' | 'bold' | 'color' | 'dir-ltr' | 'dir-rtl' | 'h1' | 'h2' | 'italic' | 'marker-color' | 'p1' | 'p2' | 'underline';

@customElement("text-editor-control")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                    border-radius: 50%;
                }
                :host(:hover) {
                    background-color: var(--light-blue-1);
                }
                :host([active]) {
                    background-color: var(--main-blue);
                }
            `,
        ];
    }

    @property()
    type: ControlType = 'h1';

    @property({type: Boolean, reflect: true})
    active = false;

    render() {
        const path = `module/_common/edit/widgets/sidebar/text-editor-controls/${ this.type.toLowerCase() }${ this.active ? '-active' : '' }.svg`;
        return html`
            <button-collection-item>
                <img-ui path="${path}"></img-ui>
            </button-collection-item>
        `;
    }
}
