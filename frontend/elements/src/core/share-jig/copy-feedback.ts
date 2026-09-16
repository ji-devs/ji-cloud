import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("share-copy-feedback")
export class _ extends LitElement {
    static get styles() {
        return css`
            :host {
                display: inline-block;
            }
            :host([copied]) {
                animation: copy-confirmed 400ms ease-out;
            }
            @keyframes copy-confirmed {
                0% {
                    opacity: 0.4;
                    transform: translateY(3px) scale(0.96);
                }
                60% {
                    opacity: 1;
                    transform: translateY(-1px) scale(1.04);
                }
                100% {
                    opacity: 1;
                    transform: translateY(0) scale(1);
                }
            }
        `;
    }

    @property({ type: Boolean, reflect: true })
    copied: boolean = false;

    render() {
        return html`<slot></slot>`;
    }
}
