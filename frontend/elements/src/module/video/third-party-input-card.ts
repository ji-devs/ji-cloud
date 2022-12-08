import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

export type Host = "youtube";

@customElement("video-third-party-input-card")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    padding: 24px;
                    display: grid;
                    border-radius: 8px;
                    background-color: #ffffff;
                }
                img-ui {
                    height: 24px;
                    margin-bottom: 36px;
                }
                ::slotted([slot="delete"]) {
                    justify-self: end;
                }
                ::slotted([slot="clip-checkbox"]) {
                    margin-top: 12px;
                }
                .start-end {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    column-gap: 8px;
                }
            `,
        ];
    }

    @property()
    host: Host = "youtube";

    render() {
        return html`
            <img-ui path="module/video/host-youtube.png"></img-ui>
            <slot name="delete"></slot>
            <slot name="input"></slot>
            <slot name="clip-checkbox"></slot>
            <div class="start-end">
                <slot name="start-at"></slot>
                <slot name="end-at"></slot>
            </div>
        `;
    }
}
