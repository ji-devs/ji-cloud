import {
    LitElement,
    html,
    css,
    customElement,
} from "lit-element";

@customElement("help-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                    height: 100%;
                    position: relative;
                }
                button {
                    margin: 0;
                    padding: 0 24px;
                    background: none;
                    border: none;
                    line-height: 34px;
                    color: inherit;
                    cursor: pointer;
                    font-size: 13px;
                }

                button div {
                    display: flex;
                    flex-direction: row;
                }
            `,
        ];
    }

    render() {
        return html`
            <button class="anchor" @click=${this.toggleOpen}>
                <div>
                    <span>Beta version</span>
                    <img-ui path="core/page-header/nav-icon-about.svg"></img-ui>
                </div>
            </button>
        `;
    }
}
