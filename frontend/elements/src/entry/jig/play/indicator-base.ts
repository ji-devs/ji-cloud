import { LitElement, html, css, property, TemplateResult } from "lit-element";

export class IndicatorBase extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    width: 94px;
                    height: 98px;
                    display: inline-grid;
                    justify-content: center;
                    align-content: space-evenly;
                    border-radius: 16px;
                    box-shadow: 0 0 6px 0 rgba(0, 0, 0, 0.16);
                    border: solid 2px var(--light-blue-4);
                    background-color: var(--light-blue-2);
                    box-sizing: border-box;
                    text-align: center;
                }
                img-ui {
                    display: grid;
                    place-content: center;
                }
                .value {
                    font-size: 22px;
                    font-weight: 500;
                    color: var(--dark-blue-8);
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
