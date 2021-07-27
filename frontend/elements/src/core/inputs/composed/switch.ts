import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("input-switch")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                input {
                    display: none;
                }
                label {
                    display: inline-grid;
                    width: 40px;
                    height: 20px;
                    align-items: center;
                    cursor: pointer;
                }
                label .track {
                    grid-row: 1;
                    grid-column: 1;
                    height: 12px;
                    width: 100%;
                    background-color: var(--light-gray-2);
                    border-radius: 6px;
                }
                label .circle {
                    grid-row: 1;
                    grid-column: 1;
                    height: 20px;
                    width: 20px;
                    border-radius: 50%;
                    background-color: var(--light-gray-4);
                    transition: transform .3s, background-color .3s;
                }
                input:checked + label .circle {
                    transform: translateX(20px);
                    background-color: var(--main-blue);
                }
            `,
        ];
    }

    @property({ type: Boolean })
    enabled: boolean = false;

    private toggle() {
        this.enabled = !this.enabled;
        this.dispatchEvent(
            new CustomEvent("custom-toggle", {
                detail: {
                    value: this.enabled,
                },
            })
        );
    }

    render() {
        return html`
            <input id="input" type="checkbox" ?checked="${this.enabled}" @change="${this.toggle}">
            <label for="input">
                <span class="track"></span>
                <span class="circle"></span>
            </label>
        `;
    }
}
