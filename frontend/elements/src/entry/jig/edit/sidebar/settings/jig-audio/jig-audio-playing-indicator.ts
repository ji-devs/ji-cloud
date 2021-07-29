import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

export type Mode = "play" | "pause";

@customElement("jig-audio-playing-indicator")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    align-items: center;
                    column-gap: 2px;
                }
                .bar {
                    height: 14px;
                    width: 2px;
                    background-color: var(--dark-blue-2);
                    animation: line 500ms ease-in-out infinite alternate;
                }
                .bar:nth-child(5n+1) {
                    animation-delay: 100ms;
                }
                .bar:nth-child(5n+2) {
                    animation-delay: 200ms;
                }
                .bar:nth-child(5n+3) {
                    animation-delay: 300ms;
                }
                .bar:nth-child(5n+4) {
                    animation-delay: 400ms;
                }
                @keyframes line {
                    from {
                        transform: scaleY(1);
                    }
                    to {
                        transform: scaleY(.5);
                    }
                }
            `,
        ];
    }

    render() {
        return html`
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
            <span class="bar"></span>
        `;
    }
}
