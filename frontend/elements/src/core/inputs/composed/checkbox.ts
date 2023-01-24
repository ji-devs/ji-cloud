import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("input-checkbox")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                label {
                    display: inline-grid;
                    grid-template-columns: auto auto;
                    column-gap: 12px;
                    justify-content: start;
                    padding: 2px;
                    font-size: 14px;
                    line-height: 30px;
                }
                /* @media (min-width: 1024px) {
                    label {
                        font-size: 16px;
                    }
                } */
                :host([error]:not([error=''])) label {
                    border: solid 1px #f00813;
                    background-color: #fff4f4;
                    border-radius: 14px;
                }
                .error {
                    display: none;
                }
                :host([error]:not([error=''])) .error {
                    display: block;
                    grid-column: 1 / -1;
                    color: var(--red-alert);
                    font-size: 14px;
                    font-weight: 500;
                    margin: 0px 8px;
                }
            `,
        ];
    }

    onChange(evt: Event) {
        const { checked } = evt.target as any;
        this.checked = checked;

        this.dispatchEvent(
            new CustomEvent("custom-toggle", {
                detail: { value: checked },
            })
        );
    }

    @property({ type: Boolean })
    checked: boolean = false;

    @property({ type: Boolean })
    indeterminate: boolean = false;

    @property()
    label: string = "";

    @property({ reflect: true })
    error: string = "";

    @property({ type: Boolean })
    disabled: boolean = false;

    render() {
        return html`
            <label>
                <input ?disabled=${this.disabled} type="checkbox" .checked=${this.checked} .indeterminate=${this.indeterminate} @change="${this.onChange}" />
                <slot name="label">${this.label}</slot>
            </label>
            <p class="error">${this.error}</p>
        `;
    }
}
