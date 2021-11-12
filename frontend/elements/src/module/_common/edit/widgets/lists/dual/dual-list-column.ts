import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import { arrayIndex } from "@utils/array";

export type SIDE = "left" | "right";

@customElement("sidebar-widget-dual-list-column")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host([side="left"]) > header {
                    border-top-left-radius: 16px;
                }
                :host([side="right"]) > header {
                    border-top-right-radius: 16px;
                }
                :host([side="left"]) > .list {
                    border-bottom-left-radius: 16px;
                }
                :host([side="right"]) > .list {
                    border-bottom-right-radius: 16px;
                }
                header {
                    background-color: var(--light-blue-2);
                    text-align: center;
                    color: var(--main-blue);
                    font-weight: 500;
                    padding-top: 9px;
                    padding-bottom: 9px;
                }
                .list {
                    border: solid 2px var(--light-blue-4);
                    background-color: var(--white);
                    display: flex;
                    flex-direction: column;
                }

                ::slotted(*:not(:last-child)) {
                    border-bottom: solid 1px var(--light-blue-4);
                }
                input {
                    outline: none;
                    border: none;
                    font-size: 16px;
                    text-align: center;
                }
            `,
        ];
    }

    @property({ reflect: true })
    side: SIDE = "left";

    @property()
    header: string = "";

    render() {
        const { header } = this;
        return html`
            <header>${header}</header>
            <div class="list">
                <slot></slot>
            </div>
        `;
    }
}
