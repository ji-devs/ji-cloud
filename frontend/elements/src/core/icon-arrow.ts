import { LitElement, html, css, customElement, property } from 'lit-element';

export type Direction = "up" | "down" | "left" | "right";

@customElement('icon-arrow')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: inline-grid;
            }
            :host([direction=right]) {
                transform: rotate(180deg);
            }
            :host([direction=up]) {
                transform: rotate(-90deg);
            }
            :host([direction=down]) {
                transform: rotate(90deg);
            }
            svg {
                fill: currentColor;
            }
        `];
    }

    @property({reflect: true})
    direction: Direction = "left";

    render() {
        return html`
            <svg
                version="1.1"
                xmlns="http://www.w3.org/2000/svg"
                width="26px"
                height="40px"
            >
                <path d="M22,40c2.2,0,4-1.9,4-4.1c0-1-0.4-2-1.2-2.8L11.3,20L24.8,6.8c1.5-1.6,1.5-4.2-0.1-5.7c-1.6-1.5-4-1.5-5.6,0L0,20l19.2,18.8C19.9,39.6,20.9,40,22,40z"/>
            </svg>
        `;
    }
}
