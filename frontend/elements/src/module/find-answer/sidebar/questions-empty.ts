import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

@customElement("questions-empty")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    --image-offset: 0px;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    padding: 8px;
                    grid-gap: 8px;
                }

                .clear {
                    margin-top: 88px;
                    align-self: flex-end;
                    width: 100%;
                    display: flex;
                    justify-content: center;
                }

                .tip {
                    border-radius: 16px;
                    background-color: var(--light-blue-2);
                    text-align: center;
                    padding: 8px;
                }

                .block {
                    height: 256px;
                    border-radius: 16px;
                    background-color: var(--light-blue-2);
                    display: flex;
                    flex-direction: column;
                    align-content: center;
                    align-items: center;
                    justify-content: center;
                }

                .block p {
                    margin: 0 64px;
                    text-align: center;
                }

                .break {
                    font-size: 20px;
                    font-weight: 500;
                    text-align: center;
                    color: var(--dark-gray-6);
                    margin: 16px 0;
                }
                .block img-ui {
                    max-width: 94%;
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

    render() {
        return html`
                <div class="tip">
                    Add and edit questions here
                </div>
                <div class="block">
                    <slot></slot>
                    <p>Add a question</p>
                </div>
        `;
    }
}
