import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

@customElement("color-select-item")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    box-sizing: border-box;
                    border-radius: 50%;
                    display: inline-grid;
                    padding: 3px;
                    position: relative;
                    cursor: pointer;
                    height: 54px;
                    width: 54px;
                }
                @media (min-width: 1920px) {
                    :host {
                        height: 56px;
                        width: 56px;
                    }
                }
                :host(:focus) {
                    outline: none;
                }
                :host([selected]),
                :host(.deleting) {
                    box-shadow: var(--main-blue) 0px 0px 0pt 3px;
                }
                :host::before {
                    grid-row: 1;
                    grid-column: 1;
                    content: "";
                    height: 100%;
                    width: 1px;
                    display: block;
                    margin: auto;
                    background: var(--light-gray-4);
                    transform: rotate(45deg);
                }
                .color-item {
                    grid-row: 1;
                    grid-column: 1;
                    z-index: 1;
                    border-radius: 50%;
                    border: solid 1px var(--light-gray-4);
                }
                ::slotted(button-icon) {
                    display: none;
                    position: absolute;
                    top: -8px;
                    right: -8px;
                }
                :host(.deleting) ::slotted(button-icon) {
                    display: inline-block;
                }
            `,
        ];
    }

    @property({ type: String })
    color?: string;

    @property({ type: Boolean, reflect: true })
    selected: boolean = false;

    @property({ type: Boolean, reflect: true })
    deletable: boolean = false;

    private onRightClick(e: MouseEvent) {
        if (!this.deletable) return;

        e.preventDefault();
        this.classList.add("deleting");
        this.tabIndex = -1;
        this.focus();

        this.addEventListener("focusout", () => {
            this.classList.remove("deleting");
        });
    }

    render() {
        return html`
            <div
                class="color-item"
                style="background-color: ${this.color}"
                @contextmenu="${(e: any) => this.onRightClick(e)}"
            >
                ${this.deletable
                    ? html`<slot name="delete-button"></slot>`
                    : nothing}
            </div>
        `;
    }
}
