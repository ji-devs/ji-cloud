import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

export type ProgressColor = "blue" | "green";

@customElement("progress-bar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    --height: 24px;
                    --border-radius: calc(var(--height) / 2);
                    display: block;
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
                .wrapper {
                    height: 100%;
                    border-radius: var(--border-radius);
                    background-color: var(--background-color);
                    overflow: hidden;
                }
                .bar {
                    height: 100%;
                    background-color: var(--color);
                }
                ::slotted([slot="progress-label"]) {
                    margin-top: 8px;
                }
                :host([progress=infinite]) .bar {
                    width: 25%;
                    animation-name: slide;
                    animation-duration: 1s;
                    animation-iteration-count: infinite;
                    animation-timing-function: linear;
                }

                @keyframes slide {
                    from {
                        transform: translateX(-100%);
                    }
                    to {
                        transform: translateX(500%);
                    }
                }
            `,
        ];
    }

    @property({ type: Number, reflect: true })
    progress: number | "infinite" = 100;

    @property({ reflect: true })
    color: ProgressColor = "blue";

    render() {
        return html`
            ${this.progress !== "infinite" ? html`
                <style>
                    .bar {
                        width: ${this.progress}%;
                    }
                </style>
            ` : nothing}
            <div class="wrapper">
                <div class="bar"></div>
            </div>
            <slot name="progress-label"></slot>
        `;
    }
}
