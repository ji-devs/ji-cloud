import { LitElement, html, css, customElement } from "lit-element";

const STR_DEFINE_DIRECTION = "Define direction of play";
const STR_PLAY_JIG = "Play JIG:";

@customElement("sidebar-jig-settings")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                h2 {
                    font-size: 18px;
                    font-weight: normal;
                    margin: 0;
                    margin-bottom: 38px;
                }
                h3 {
                    color: var(--main-blue);
                    font-size: 18px;
                    font-weight: normal;
                    margin-bottom: 16px;
                }
                .direction-options {
                    margin-left: 16px;
                    display: grid;
                    row-gap: 8px;
                }
            `,
        ];
    }

    render() {
        return html`
            <h2>${STR_DEFINE_DIRECTION}</h2>
            <h3>${STR_PLAY_JIG}</h3>
            <div class="direction-options">
                <slot name="direction-options"></slot>
            </div>
        `;
    }
}
