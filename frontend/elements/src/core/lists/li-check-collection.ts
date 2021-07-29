import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/overlays/anchored-overlay";

@customElement('li-check-collection')
export class _ extends LitElement {
    static get styles() {
        return [css`
            anchored-overlay {
                width: 100%;
            }
            li {
                margin-bottom: 4px;
                list-style-type:  none;
                display: flex;
                justify-content: space-between;
                align-items:  center;
            }
            li:hover, :host([open]) li {
                background-color: #e7f0fe;
            }
            p {
                padding-left: 20px;
                padding-right: 20px;
                margin: 0;
            }
            img-ui {
                display: block;
                padding-right: 20px;
            }

            anchored-overlay::part(overlay) {
                padding: 16px 0;
                width: 100%;
                border-radius: 14px;
                box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                background-color: #fff;
                z-index: 1;
            }
        `];
    }

    @property({type: Boolean, reflect: true})
    open: boolean = false;

    public toggleOpen() {
        this.open = !this.open;
    }

    render() {
        return html`
            <anchored-overlay
                ?open="${this.open}"
                @close="${() => this.open = false}"
                positionY="top-in"
                positionX="right-out"
            >
                <li slot="anchor" @click=${() => this.toggleOpen()}>
                    <p><slot name="label"></slot></p>
                    <img-ui path="core/lists/angle-right.svg"></img-ui>
                </li>
                <div class="overlay" slot="overlay">
                    <slot></slot>
                </div>
            </anchored-overlay>
        `;
    }
}
