import { LitElement, html, css, customElement } from "lit-element";

const STR_TITLE = "Current JIG Theme";

@customElement("theme-selector-jig")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: absolute;
                    top: -3px;
                    left: -4px; /*not sure why, otherwise it's off center*/
                    border-radius: 16px;
                    border: solid 3px var(--light-orange-3);
                    background-color: var(--light-orange-1);
                    display: flex;
                    justify-content: center;
                    box-sizing: border-box;
                    width: 168px;
                    height: 150px;
                }
                @media (min-width: 1920px) {
                    :host {
                        width: 232px;
                        height: 197px;
                    }
                }

                .title-container {
                    border-radius: 16px;
                    margin-top: -16px;
                    width: 158px;
                    height: 32px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.08);
                    border: solid 3px #fdd994;
                    background-color: var(--light-orange-3);
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }
                .title {
                    color: var(--dark-gray-6);
                    font-weight: 600;
                    font-size: 12px;
                }
                @media (min-width: 1920px) {
                    .title {
                        font-size: 14px;
                    }
                }
            `,
        ];
    }
    render() {
        return html` <div class="title-container">
            <div class="title">${STR_TITLE}</div>
        </div>`;
    }
}
