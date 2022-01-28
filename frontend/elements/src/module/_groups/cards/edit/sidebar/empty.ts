import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

@customElement("sidebar-empty")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    height: 100%;
                }
                img-ui {
                    margin-bottom: 24px;
                    transform: translateX(32px);
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
    label: string = "";

    render() {
        return html`
            <section>
                <img-ui
                    path="module/_groups/cards/edit/sidebar/edit-words.svg"
                ></img-ui>
                <div class="label">${this.label}</div>
            </section>
            <div class="clear"><slot name="clear"></slot></div>
        `;
    }
}
