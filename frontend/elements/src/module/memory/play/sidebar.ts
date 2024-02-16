import {LitElement, html, css, customElement, property} from "lit-element";

@customElement("play-sidebar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .wrapper {
                    width: calc(263rem * (1920 / 1719));
                    height: calc(915rem * (1920 / 1719));
                    margin-left: calc(24rem * (1920 / 1719));
                    margin-top: calc(24rem * (1920 / 1719));
                    border-radius: 16px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: rgba(10, 48, 82, .2);
                    position: relative;
                }
                .pairs {
                    width: calc(calc(263rem * (1920 / 1719)) + 1474rem + 150rem);
                    height: calc(915rem * (1920 / 1719));
                    position: relative;
                    overflow-x: hidden;
                    overflow-y: scroll;
                    pointer-events: none;
                }
                :host([animating]) .pairs {
                    overflow-x: visible;
                    overflow-y: hidden;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    animating: boolean = false;

    render() {
        return html`
            <div class="wrapper">
                <div class="pairs">
                    <slot></slot>
                </div>
            </div>
        `;
    }
}
