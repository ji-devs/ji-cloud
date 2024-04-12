import { LitElement, html, css, customElement, property } from "lit-element";

export type Effect = "appear" | "fade-in-top" | "fade-in-bottom" | "fade-in-left" | "fade-in-right";

@customElement("animations-hide")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                    --translation-x: 0;
                    --translation-y: 0;
                }
                :host([effect=fade-in-top]) {
                    --translation-y: -200%;
                }
                :host([effect=fade-in-bottom]) {
                    --translation-y: 200%;
                }
                :host([effect=fade-in-left]) {
                    --translation-x: -200%;
                }
                :host([effect=fade-in-right]) {
                    --translation-x: 200%;
                }
                .wrapper {
                    display: inline-block;
                    transition: transform .2s, opacity .2s;
                }
                :host([visible]) .wrapper {
                    transform: translate(0%);
                    opacity: 1;
                }
                :host(:not([visible])) .wrapper {
                    transform: translate(var(--translation-x), var(--translation-y));
                    opacity: 0;
                    pointer-events: none;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    visible = true;

    @property({ reflect: true })
    effect: Effect = "appear";

    render() {
        return html`
            <div class="wrapper">
                <slot></slot>
            </div>
        `;
    }
}
