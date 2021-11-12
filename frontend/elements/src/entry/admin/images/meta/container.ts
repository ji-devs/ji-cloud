import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/inputs/composed/checkbox";
import "@elements/core/inputs/composed/text-underline";
import "@elements/core/inputs/composed/textarea-underline";
import "@elements/core/buttons/rectangle";
import "@elements/core/dividers/horizontal-full";

const STR_TITLE = "Label Images";

@customElement("image-meta-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    margin-top: 29px;
                    padding-left: 40px;
                    padding-right: 40px;
                }
                .header {
                }
                .main {
                    /* Couldn't get this to work... */
                    overflow-y: auto;
                }
                article {
                    display: grid;
                    grid-template-columns: 288px 1fr;
                    grid-template-rows: auto 1fr;
                    column-gap: 10px;
                }
                .image {
                    display: flex;
                    justify-content: center;
                }
                .actions {
                    display: flex;
                    justify-content: space-between;
                    margin-top: 5px;
                    margin-bottom: 20px;
                }

                .actions > .buttons {
                    display: flex;
                }

                .premium {
                    justify-self: start;
                }

                .left {
                    grid-column: 1;
                    grid-row: 1;
                }
                .right {
                    grid-column: 2;
                    grid-row: 1;
                }
                .bottom {
                    grid-column: 1 / -1;
                    grid-row: 2;
                    display: flex;
                    padding-top: 10px;
                    justify-content: flex-end;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="header">
                <slot name="header"></slot>
            </div>
            <div class="main">
                <article>
                    <div class="left">
                        <div class="image">
                            <slot name="image"></slot>
                        </div>
                        <div class="actions">
                            <div class="premium">
                                <slot name="premium"></slot>
                            </div>
                            <div class="buttons">
                                <slot name="replace"></slot>
                                <horizontal-full color="blue"></horizontal-full>
                                <slot name="delete"></slot>
                            </div>
                        </div>
                        <slot name="divider"></slot>
                        <slot name="description"></slot>
                    </div>
                    <div class="right">
                        <slot name="right"></slot>
                    </div>
                    <div class="bottom">
                        <slot name="next"></slot>
                    </div>
                </article>
            </div>
            <slot name="modal"></slot>
        `;
    }
}
