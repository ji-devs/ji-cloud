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
                    padding: 32px 32px;
                    display: grid;
                    row-gap: 10px;
                    border-radius: 8px;
                    background-color: #ffffff;
                }
                img-ui {
                    height: 32px;
                    margin-bottom: 46px;
                }
                ::slotted([slot="delete"]) {
                    justify-self: start;
                }
            `,
        ];
    }

    @property()
    host: Host = "youtube";

    render() {
        return html`
            <img-ui path="module/video/host-youtube.png"></img-ui>
            <slot name="input"></slot>
            <slot name="delete"></slot>
        `;
    }
}
