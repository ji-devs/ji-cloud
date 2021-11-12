import {
    LitElement,
    svg,
    html,
    css,
    customElement,
    property,
} from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

@customElement("trace-edit-reshape-menu")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: absolute;
                }
                ::slotted([slot="close"]) {
                    display: block;
                    position: absolute;
                    top: -12px;
                    left: -12px;
                    width: 24px;
                    height: 24px;
                }
                section {
                    height: 48px;
                    display: flex;
                    border-radius: 8px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: var(--white);
                    align-items: center;
                    gap: 16px;
                    padding-left: 16px;
                }

                :host([noGap]) section {
                    gap: 0;
                    padding-left: 0;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    noGap: boolean = false;

    render() {
        return html`
            <slot name="close"></slot>
            <section>
                <slot></slot>
            </section>
        `;
    }
}
