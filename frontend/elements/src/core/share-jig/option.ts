import { LitElement, html, css, customElement, property } from "lit-element";

export type Kind = "code" | "google-classroom" | "ms-teams" | "embed" | "copy" | "share";

@customElement("share-jig-option")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: auto 1fr auto;
                    align-items: center;
                    column-gap: 10px;
                    padding: 6px;
                    cursor: pointer;
                    font-size: 14px;
                }
                :host(:hover) {
                    background-color: var(--light-blue-1);
                }
                .help {
                    height: 20px;
                    width: 20px;
                    display: inline-grid;
                    place-content: center;
                    color: var(--main-blue);
                    background-color: var(--light-blue-2);
                    border-radius: 50%;
                }
                img-ui {
                    width: 24px;
                }
            `,
        ];
    }

    @property()
    kind: Kind = "code";

    render() {

        return html`
            <slot slot="back" name="back"></slot>
            <slot slot="close" name="close"></slot>
            <img-ui path="core/share-jig/${this.kind}.svg"></img-ui>
            <span class="label"><slot></slot></span>
            <!-- <span class="help">?</span> -->
        `;
    }
}
