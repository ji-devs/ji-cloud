import { LitElement, html, css, property, TemplateResult } from "lit-element";

export class IndicatorBase extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    width: 94px;
                    height: 98px;
                    justify-items: center;
                    display: inline-grid;
                    justify-content: center;
                    align-content: space-evenly;
                    border-top-left-radius: 50%;
                    border-bottom-left-radius: 50%;
                    box-shadow: 0 0 6px 0 rgba(0, 0, 0, 0.16);
                    border: solid 2px var(--light-blue-4);
                    background-color: var(--light-blue-2);
                    box-sizing: border-box;
                    text-align: center;
                }
                img-ui {
                    height: 30px;
                    width: 30px;
                    display: grid;
                    place-content: center;
                }
                .value {
                    font-size: 22px;
                    font-weight: 500;
                    color: var(--dark-blue-8);
                }

                /* mobile */
                @media (max-width: 1000px) {
                    :host {
                        width: 50px;
                        height: 58px;
                    }
                    img-ui {
                        height: 16px;
                        width: 16px;
                    }
                    .value {
                        font-size: 10px;
                    }
                }
            `,
        ];
    }

    @property()
    value: string = "";

    renderIndicator(renderImage: () => TemplateResult) {
        return html`
            ${renderImage()}
            <span class="value">${this.value}</span>
        `;
    }
}
