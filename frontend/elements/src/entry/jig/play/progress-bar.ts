import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("jig-play-progress-bar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    box-sizing: border-box;
                    border: solid 2px var(--light-blue-3);
                    background-color: var(--dark-blue-8);
                    border-radius: 28px;
                    padding: 8px;
                }
                .bar {
                    background-color: var(--main-yellow);
                    border-radius: 10px;
                    height: 12px;
                    transition: width 0.3s;
                }

                /* mobile */
                @media (max-width: 1000px) {
                    :host {
                        padding: 4px;
                    }
                    .bar {
                        height: 6px;
                    }
                }
            `,
        ];
    }

    @property({ type: Number })
    percent: number = 1;

    render() {
        return html`
            <style>
                .bar {
                    width: ${this.percent}%;
                }
            </style>
            <div class="bar"></div>
        `;
    }
}
