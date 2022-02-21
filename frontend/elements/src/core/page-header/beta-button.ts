import {
    LitElement,
    html,
    css,
    customElement,
    property,
    internalProperty
} from "lit-element";

@customElement("beta-button")
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
                    font-weight: 600;
                }

                button div {
                    display: flex;
                    flex-direction: row;
                }

                img-ui {
                    display: flex;
                    margin-left: 8px;
                }
            `,
        ];
    }

    @internalProperty()
    private open = false;

    private toggleOpen() {
        this.open = !this.open;
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

