import { LitElement, html, css, customElement } from "lit-element";

@customElement("animations-hover")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                }
                :hover(:hover) {
                    background-color: green;
                }
                :host(.grow) {
                    transition: .3s scale;
                }
                :host(.grow:hover) {
                    scale: 1.1;
                }
                :host(.tilt) {
                    transition: .3s rotate;
                }
                :host(.tilt:hover) {
                    rotate: 4deg;
                }
                @keyframes buzz {
                    10%  { rotate:  5deg; }
                    20%  { rotate: -4deg; }
                    30%  { rotate:  4deg; }
                    40%  { rotate: -3deg; }
                    50%  { rotate:  3deg; }
                    60%  { rotate: -2deg; }
                    70%  { rotate:  2deg; }
                    80%  { rotate: -1deg; }
                    90%  { rotate:  1deg; }
                    100% { rotate:  0deg; }
                }
                :host(.buzz:hover) {
                    animation-name: buzz;
                    animation-duration: 1s;
                    animation-iteration-count: 1;
                }
            `,
        ];
    }

    render() {
        return html`
            <slot></slot>
        `;
    }
}
