import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("main-card-pair")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                section,
                .index {
                    width: 350px;
                }

                section {
                    border-radius: 24rem;
                    height: 236px;
                }

                section.hover {
                    background-color: #deecff;
                }

                section.hover > .close {
                    display: block;
                }

                .close {
                    display: none;
                    position: relative;
                    top: 0;
                    left: 0;
                }
                slot[name="close"]::slotted(*) {
                    position: absolute;
                    top: -16px;
                    left: 333px;
                    display: inline-block;
                    width: 32px;
                    height: 32px;
                }

                .cards {
                    display: grid;
                    padding: 27px;
                    grid-template-columns: 1fr 24px 1fr;
                    grid-template-rows: 24px 1fr 24px;
                }
                .left {
                    grid-row: 1 / span 2;
                    grid-column: 1 / span 2;
                }
                .right {
                    grid-row: 2 / span 2;
                    grid-column: 2 / span 2;
                }
                .index {
                    font-size: 14px;
                    text-align: center;
                }
            `,
        ];
    }

    updated(changed: any) {
        if (typeof changed.get("hoverLock") === "boolean") {
            if (!this.hoverLock) {
                this.hover = false;
            }
        }

        if (typeof changed.get("hover") === "boolean") {
            const { hoverLock } = this;
            if (hoverLock) {
                this.hover = true;
            }
        }
    }

    onEnter() {
        if (this.hoverable) {
            this.hover = true;
        }
    }

    onLeave() {
        if (this.hoverable) {
            this.hover = false;
        }
    }

    @property({ type: Boolean })
    hover: boolean = false;

    @property({ type: Number })
    index: number = 0;

    @property({ type: Boolean })
    hoverable: boolean = false;

    @property({ type: Boolean })
    hoverLock: boolean = false;

    render() {
        const { hover, index } = this;

        return html`
            <section
                class="${classMap({ hover })}"
                @mouseenter="${this.onEnter}"
                @mouseleave="${this.onLeave}"
            >
                <div class="close"><slot name="close"></slot></div>
                <div class="cards">
                    <div class="right"><slot name="right"></slot></div>
                    <div class="left"><slot name="left"></slot></div>
                </div>
            </section>
            <div class="index">${index + 1}</div>
            <slot name="error"></slot>
        `;
    }
}
