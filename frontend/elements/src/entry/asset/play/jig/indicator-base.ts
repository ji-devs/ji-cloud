import { LitElement, html, css, property, TemplateResult } from "lit-element";

export class IndicatorBase extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    justify-items: center;
                    display: inline-grid;
                    justify-content: center;
                    align-content: space-evenly;
                    padding-left: 10px;
                    border-top-left-radius: 50%;
                    border-bottom-left-radius: 50%;
                    box-shadow: 0 0 6px 0 rgba(0, 0, 0, 0.16);
                    border: solid 2px var(--light-blue-4);
                    background-color: var(--light-blue-2);
                    box-sizing: border-box;
                    text-align: center;
                    font-size: 18px;
                    line-height: 1em;
                    font-weight: 500;
                    color: var(--dark-blue-8);
                    width: 80px;
                    height: 72px;
                }
                @media (max-width: 1000px) {
                    :host {
                        width: 40px;
                        height: 36px;
                        font-size: 13px;
                    }
                }
                img-ui {
                    height: 28px;
                    width: 28px;
                    display: grid;
                    place-content: center;
                }
                @media (max-width: 1000px) {
                    img-ui {
                        height: 14px;
                        width: 14px;
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
