import { LitElement, html, css, customElement, property, internalProperty } from 'lit-element';

@customElement('home-search-result-details')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                --closed-height: 38px;
                display: grid;
                grid-template-columns: 1fr min-content;
                height: var(--closed-height);
                overflow: hidden;
            }
            :host([open]) {
                height: unset;
            }
            .collapse-icon::after {
                content: '>';
                cursor: pointer;
                display: inline-block;
                transform: rotate(90deg);
                transition: transform .3s;
                line-height: 38px;
            }
            :host([open]) .collapse-icon::after {
                transform: rotate(-90deg);
            }
        `];
    }

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    private toggleOpen = () => {
        this.open = !this.open;
        if (!this.open) {
            this.dispatchEvent(new Event("closed"));
        }
    }

    render() {
        return html`
            <div class="content">
                <slot></slot>
            </div>
            <span class="collapse-icon" @click="${this.toggleOpen}"></span>
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
