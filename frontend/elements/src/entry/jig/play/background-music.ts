import { LitElement, html, css, customElement, property } from "lit-element";
import { actionStyles } from "./action-styles";

@customElement("jig-play-background-music")
export class _ extends LitElement {
    static get styles() {
        return [
            actionStyles,
            css`
                :host {
                    display: inline-grid;
                }
                button {
                    grid-column: 1;
                    grid-row: 1;
                }
                :host(:not([playing]))::after {
                    content: '';
                    grid-column: 1;
                    grid-row: 1;
                    margin: 0 auto;
                    width: 2px;
                    background-color: #ffffff;
                    transform: rotate(-45deg);
                    pointer-events: none;
                }
                :host([disabled]) .action {
                    border-color: var(--light-gray-2);
                    background-color: var(--light-gray-4);
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    playing: boolean = false;

    @property({ type: Boolean, reflect: true })
    disabled: boolean = false;

    render() {
        return html`
            <button class="action small">
                <img-ui path="entry/jig/play/music.svg"></img-ui>
            </button>
        `;
    }
}
