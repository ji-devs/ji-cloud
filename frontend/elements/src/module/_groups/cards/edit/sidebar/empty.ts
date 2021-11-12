import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

const STR_LABEL = html`Edit your words<br />on the cards`;

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
                }
                .label {
                    font-size: 18px;
                    font-weight: 500;
                    line-height: 1.22;
                    text-align: center;
                    color: var(--dark-gray-6);
                    margin-left: -16px;
                }

                section {
                    display: flex;
                    flex-direction: column;
                    justify-content: flex-start;
                    align-items: flex-start;
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

    render() {
        return html`
            <section>
                <img-ui
                    path="module/_groups/cards/edit/sidebar/jiggling-card-pointer.svg"
                ></img-ui>
                <div class="label">${STR_LABEL}</div>
            </section>
            <div class="clear"><slot name="clear"></slot></div>
        `;
    }
}
