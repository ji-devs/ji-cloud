import {
    LitElement,
    html,
    css,
    customElement,
    property,
    unsafeCSS,
} from "lit-element";
import { mediaUi } from "@utils/path";

const STR_TITLE = "My Pairs";

@customElement("play-sidebar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                }
                section {
                    width: calc(263rem * (1920 / 1719));
                    height: calc(915rem * (1920 / 1719));
                    margin-left: calc(24rem * (1920 / 1719));
                    margin-top: calc(24rem * (1920 / 1719));
                    border-radius: 16px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: var(--dark-blue-7);
                }

                .title {
                    padding-top: calc(16rem * (1920 / 1719));
                    font-size: calc(22rem * (1920 / 1719));
                    text-align: center;
                    color: var(--light-gray-4);
                }
                .pairs {
                }
            `,
        ];
    }

    render() {
        return html` <section>
            <div class="title">${STR_TITLE}</div>
            <div class="pairs">
                <slot></slot>
            </div>
        </section>`;
    }
}
