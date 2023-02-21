import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("jig-play-progress-bar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    box-sizing: border-box;
                    border: solid 2px #fff;
                    background-color: var(--dark-blue-8);
                    border-radius: 28px;
                    padding: 4px;
                }
                .bar {
                    background-color: var(--main-yellow);
                    border-radius: 10px;
                    height: 6px;
                    transition: width 0.3s;
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
