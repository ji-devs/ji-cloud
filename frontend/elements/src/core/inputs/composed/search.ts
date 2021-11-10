import { LitElement, html, css, customElement, property, query } from "lit-element";

@customElement("input-search")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    min-width: 200px;
                }
                .wrapper {
                    display: grid;
                    grid-template-columns: 1fr 32px;
                    align-items: center;
                    height: 32px;
                    border-radius: 18px;
                    border: solid 1px #e5e7ef;
                    background-color: #f8f9fd;
                    box-sizing: border-box;
                }
                input {
                    border: 0;
                    font-size: 16px;
                    padding: 0;
                    margin-left: 8px;
                }
                input:focus {
                    outline: none;
                }
                button {
                    all: unset;
                    display: inline-grid;
                    justify-content: center;
                    cursor: pointer;
                }
                button fa-icon {
                    color: #a9b1b5;
                    font-size: 14px;
                }
                button:hover fa-icon,
                button:active fa-icon {
                    color: var(--main-blue);
                }
                input::-webkit-search-decoration,
                input::-webkit-search-cancel-button,
                input::-webkit-search-results-button,
                input::-webkit-search-results-decoration {
                    display: none;
                }
            `,
        ];
    }

    @property()
    placeholder: string = "";

    @property()
    value: string = "";

    @query("input")
    input!: HTMLInputElement;

    onSubmit(e: Event) {
        e.preventDefault();

        this.value = this.input.value;

        this.dispatchEvent(
            new CustomEvent("custom-search", {
                detail: { query: this.value },
                composed: true,
                bubbles: true,
            })
        );
    }

    render() {
        return html`
            <form class="wrapper" @submit="${this.onSubmit}">
                <input
                    type="search"
                    value="${this.value}"
                    placeholder="${this.placeholder}"
                />
                <button type="submit">
                    <fa-icon icon="fa-regular fa-magnifying-glass"></fa-icon>
                </button>
            </form>
        `;
    }
}
