import { LitElement, html, css, customElement, property } from "lit-element";

const STR_DRAG_ME = "Drag";

@customElement("menu-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                    grid-template-columns: 24px auto;
                }
                .drag-ui {
                    user-select: none;
                    background-color: var(--main-blue);
                    clip-path: path("M 80 0 c -2 19 -2 24 -14 24 L 14 24 c -12 0 -12 -5 -14 -24 z");
                    width: 80px;
                    font-size: 14px;
                    line-height: 24px;
                    text-align: center;
                    color: #ffffff;
                    rotate: 90deg;
                    transform-origin: center;
                    place-self: center;
                }
                .main {
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.2);
                    border-radius: 8px;
                    padding: 14px 16px;
                    background-color: var(--white);
                }
            `,
        ];
    }

    render() {
        return html`
            <!-- tab-index needed to make drag-ui draggable -->
            <div class="drag-ui" tabindex="0">
                <fa-icon icon="fa-solid fa-grip-dots"></fa-icon>
                ${STR_DRAG_ME}
            </div>
            <div class="main">
                <slot></slot>
            </div>
        `;
    }
}
