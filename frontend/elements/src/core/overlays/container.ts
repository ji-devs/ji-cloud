import { LitElement, html, css, customElement, property } from "lit-element";

import { queryPierceShadow } from "@utils/dom";
@customElement("overlay-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    position: fixed;
                    top: 0;
                    left: 0;
                    z-index: 2;
                }
            `,
        ];
    }

    /// this allows rendering overlay-container anywhere in the tree
    /// and it will propogate to the top (either into #overlay if that exists, or body)
    /// breaks with frameworks though...
    @property({ type: Boolean })
    reparent: boolean = false;

    firstUpdated() {
        if (!this.reparent) {
            return;
        }

        let parentElement = queryPierceShadow(document, "#overlay");
        if (!parentElement) {
            console.warn("couldn't find #overlay! using document.body");
            parentElement = document.body;
        }

        parentElement.appendChild(this);
    }

    render() {
        return html` <slot></slot> `;
    }
}
