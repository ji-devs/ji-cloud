import {
    LitElement,
    html,
    css,
    customElement,
    property,
    internalProperty,
} from "lit-element";

@customElement("home-search-result-details")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    --closed-height: 38px;
                    display: grid;
                    grid-template-columns: 1fr min-content;
                    align-items: start;
                    height: var(--closed-height);
                    overflow: hidden;
                }
                :host([open]) {
                    height: unset;
                }
                button-empty {
                    height: var(--closed-height);
                    display: inline-grid;
                    place-content: center;
                }
                .collapse-icon {
                    display: inline-block;
                    font-size: 14px;
                    text-align: center;
                    transform: rotate(0deg);
                    transition: transform 0.3s;
                    width: 14px;
                }
                :host([open]) .collapse-icon {
                    transform: rotate(90deg);
                }

                .content {
                    /* padding: 10px 0; */
                    display: grid;
                    row-gap: 7px;
                }

                ::slotted(h4) {
                    line-height: var(--closed-height);
                }

                ::slotted(h4), ::slotted(p) {
                    margin: 0;
                }

                ::slotted(div) {
                    display: flex;
                    flex-wrap: wrap;
                    column-gap: 7px;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    private toggleOpen = () => {
        this.open = !this.open;
        if (!this.open) {
            this.dispatchEvent(new Event("closed"));
        }
    };

    render() {
        return html`
            <div class="content" @click=${this.toggleOpen}>
                <slot></slot>
            </div>
            <button-empty @click=${this.toggleOpen}>
                <fa-icon
                    class="collapse-icon"
                    icon="fa-regular fa-chevron-right"
                ></fa-icon>
            </button-empty>
        `;
    }
}

// import { LitElement, html, css, customElement, property } from 'lit-element';

// @customElement('home-search-result-details')
// export class _ extends LitElement {
//     static get styles() {
//         return [css`
//             details {
//                 padding: 12px 0;
//             }
//             summary {
//                 list-style: none;
//                 display: flex;
//                 justify-content: space-between;
//                 cursor: pointer;
//                 font-size: 14px;
//                 font-weight: 500;
//             }
//             summary::after {
//                 content: '>';
//                 transform: rotate(90deg);
//                 transition: transform .3s;
//             }
//             ::slotted([slot=summary]) {
//                 margin: 0;
//                 display: inline-block;
//             }
//             details[open] summary::after {
//                 transform: rotate(-90deg);
//             }
//         `];
//     }

//     render() {
//         return html`
//             <details>
//                 <summary><slot name="summary"></slot></summary>
//                 <slot name="details"></slot>
//             </details>
//         `;
//     }
// }
