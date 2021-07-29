import { LitElement, html, css, property } from "lit-element";

export class IndicatorBase extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                }
                .background {
                    grid-column: 1;
                    grid-row: 1;
                    display: inline-grid;
                    justify-items: center;
                    grid-template-rows: 30px 2px 54px;
                    box-sizing: border-box;
                }
                .background .top {
                    background-color: var(--dark-blue-8);
                    border: solid var(--light-blue-3) 2px;
                    border-bottom: none;
                    border-radius: 40px 40px 0 0;
                    grid-column: 1;
                    grid-row: 1 / span 2;
                    width: 46px;
                    z-index: 1;
                }
                .background .bottom {
                    background-color: var(--dark-blue-8);
                    border: solid var(--light-blue-3) 2px;
                    grid-column: 1;
                    grid-row: 2 / span 2;
                    width: 88px;
                    border-radius: 12px;
                }
                .foreground {
                    grid-column: 1;
                    grid-row: 1;
                    display: inline-grid;
                    justify-items: center;
                    padding: 10px;
                    box-sizing: border-box;
                    z-index: 1;
                }
                .foreground .value {
                    font-size: 22px;
                    font-weight: 500;
                    color: #ffffff;
                }
            `,
        ];
    }

    @property()
    value: string = "";

    renderIndicator(image: string) {
        return html`
            <div class="background">
                <div class="top"></div>
                <div class="bottom"></div>
            </div>
            <div class="foreground">
                <img-ui path="entry/jig/play/${image}.svg"></img-ui>
                <span class="value">${this.value}</span>
            </div>
        `;
    }
}
