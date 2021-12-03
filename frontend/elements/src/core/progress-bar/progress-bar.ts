import { LitElement, html, css, customElement, property } from "lit-element";

export type ProgressColor = "blue" | "green";

@customElement("progress-bar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    --height: 24px;
                    height: var(--height);
                }
                :host([color="blue"]) {
                    --color: var(--main-blue);
                    --background-color: #c4dbfe;
                }
                :host([color="green"]) {
                    --color: #7fd29c;
                    --background-color: #d0ebda;
                }
                .outer {
                    display: grid;
                    background-color: var(--background-color);
                    height: 100%;
                    border-radius: calc(var(--height) / 2);
                    grid-template-columns: var(--height) repeat(99, 1fr);
                }
                .inner {
                    border-radius: calc(var(--height) / 2);
                    background-color: var(--color);
                }
            `,
        ];
    }

    @property({ type: Number })
    progress: number = 100;

    @property({ reflect: true })
    color: ProgressColor = "blue";

    render() {
        return html`
            <style>
                .inner {
                    grid-column: 1 / span ${this.progress + 1};
                }
            </style>
            <div class="outer">
                <div class="inner"></div>
            </div>
        `;
    }
}
