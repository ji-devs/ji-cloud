import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("search-dropdown")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .input-wrapper {
                    position: relative;
                }

                ul {
                    list-style-type: none;
                    padding: 0;
                    margin-top: -40px;
                }

                input {
                    outline: none;
                    border: none;
                    width: inherit;
                }

                input {
                    font-size: 16px;
                }

                img-ui {
                    position: absolute;
                    top: -5px;
                    right: 20px;
                    transform: rotate(180deg);
                }

                input:focus {
                    border: none;
                    outline: none;
                }

                .open {
                    display: block;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    width: 196px;
                    position: absolute;
                    left: -16px;
                    top: 70px;
                    border-radius: 0 0 14px 14px;
                    z-index: -1;
                    background-color: #ffffff;
                    padding-bottom: 10px;
                    font-size: 20px;
                }

                ::slotted(*) {
                    padding-top: 16px;
                    display: none;
                }

                .open ::slotted(*) {
                    display: block;
                }

                input::placeholder {
                    color: #272727;
                    font-size: 20px;
                }
            `,
        ];
    }

    @property()
    value: string = "";

    @property()
    placeholder: string = "";

    @property({ type: Boolean })
    open: boolean = false;

    render() {
        const { open, value, placeholder } = this;

        return html`
            <div class="input-wrapper">
                <input
                    placeholder="${placeholder}"
                    value="${value}"
                    type="text"
                    class=""
                />
                <img-ui path="icn-chevron-dropdown-up.svg"></img-ui>
                <ul class="${open ? " open" : ""}">
                    <slot></slot>
                </ul>
            </div>
        `;
    }
}
