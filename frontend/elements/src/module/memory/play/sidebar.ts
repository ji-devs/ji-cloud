import { LitElement, html, css, customElement } from "lit-element";

@customElement("play-sidebar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .pairs {
                    width: calc(263rem * (1920 / 1719));
                    height: calc(915rem * (1920 / 1719));
                    margin-left: calc(24rem * (1920 / 1719));
                    margin-top: calc(24rem * (1920 / 1719));
                    border-radius: 16px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: rgba(10, 48, 82, .2);
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="pairs">
                <slot></slot>
            </div>
        `;
    }
}
