import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

@customElement("carousel-multi")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 116px 1fr 116px;
                }

                .arrow {
                    cursor: pointer;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }

                .content-container {
                    display: flex;
                    width: 100%;
                    overflow-x: hidden;
                }
                .content {
                    display: flex;
                    gap: 20px;
                }
                ::slotted(*) {
                    flex: 0 0 auto;
                }

                .left,
                .right {
                    display: flex;
                    justify-content: center;
                }

                .edge {
                    position: relative;
                    width: 0px;
                    height: 0px;
                }
                .edge-overlay {
                    position: absolute;
                    top: 0;
                    width: 100px;
                    /* TODO - derive somehow */
                    height: 387px;
                }
                .left .edge-overlay {
                    left: 0px;
                    background-image: linear-gradient(
                        to right,
                        rgba(255, 255, 255, 1),
                        rgba(255, 255, 255, 0) 100%
                    );
                }
                .right .edge-overlay {
                    right: 0px;
                    background-image: linear-gradient(
                        to right,
                        rgba(255, 255, 255, 0),
                        rgba(255, 255, 255, 1) 100%
                    );
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="left">
                <div class="arrow">
                    <img-ui
                        path="core/_common/chevron-left-grey-large.svg"
                        alt=""
                    ></img-ui>
                </div>
                <div class="edge"><div class="edge-overlay"></div></div>
            </div>
            <div class="middle content-container">
                <div class="content">
                    <slot></slot>
                </div>
            </div>
            <div class="right">
                <div class="edge"><div class="edge-overlay"></div></div>
                <div class="arrow">
                    <img-ui
                        path="core/_common/chevron-right-grey-large.svg"
                        alt=""
                    ></img-ui>
                </div>
            </div>
        `;
    }
}
