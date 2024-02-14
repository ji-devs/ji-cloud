import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

@customElement("flashcards-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: absolute;
                    top: 0;
                    left: 0;
                    display: flex;
                    width: 100%;
                    height: 100%;
                    align-items: center;
                    justify-content: center;
                }
                div.container {
                    margin-top: 48rem; // Offset from next button
                }
                section {
                    display: flex;
                    gap: 56rem;
                }

                div.next {
                    position: relative;
                    bottom: -48rem;
                    display: flex;
                    justify-content: center;
                }

                ::slotted([slot="next"]) {
                    filter: drop-shadow(0 3rem 12rem rgba(0, 0, 0, 0.24));
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="container">
                <section>
                    <slot></slot>
                </section>
                <div class="next">
                    <slot name="next"></slot>
                </div>
            </div>
        `;
    }
}
