import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

@customElement("sidebar-empty")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    --image-offset: 0px;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    height: 100%;
                }
                img-ui {
                    margin-bottom: 24px;
                    transform: translateX(var(--image-offset));
                }
                .label {
                    font-size: 18px;
                    font-weight: 500;
                    line-height: 1.22;
                    text-align: center;
                    color: var(--dark-gray-6);
                }

                section {
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                }

                .clear {
                    margin-top: 88px;
                    align-self: flex-end;
                    width: 100%;
                    display: flex;
                    justify-content: center;
                }
            `,
        ];
    }

    @property({ type: String })
    label?: string;

    @property({ type: String })
    imagePath?: string;

    @property({ type: Number, reflect: true })
    imageOffset: number = 0;

    firstUpdated(_changed: any) {
        this.style.setProperty('--image-offset', `${this.imageOffset}px`);
    }

    renderImage() {
        return this.imagePath ? html`<img-ui path=${this.imagePath}></img-ui>` : nothing;
    }

    renderLabel() {
        return this.label ? html`<div class="label">${this.label}</div>` : nothing;
    }

    render() {
        return html`
            <section>
                ${this.renderImage()}
                ${this.renderLabel()}
            </section>
            <div class="clear"><slot name="clear"></slot></div>
        `;
    }
}
