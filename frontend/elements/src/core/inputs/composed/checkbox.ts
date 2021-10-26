import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

@customElement("input-checkbox")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                label {
                    display: flex;
                    align-items: baseline;
                    font-size: 14px;
                }
                @media (min-width: 1920px) {
                    label {
                        font-size: 16px;
                    }
                }
                input {
                    margin-right: 1px;
                    display: inline-block;
                }
                span {
                    margin-left: 12px;
                    white-space: nowrap;
                }
                .errorwrapper {
                    border: solid 1px #f00813;
                    background-color: #fff4f4;
                    border-radius: 14px;
                    padding: 2px 16px 2px 4px;
                    margin-right: 16px;
                }
                div {
                    display: flex;
                    align-items: center;
                    height: 30px;
                }
                input {
                    margin: 0;
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

    @property()
    label: string = "";

    @property()
    error: string = "";

    @property({ type: Boolean })
    disabled: boolean = false;

    render() {
        const { label, error, checked } = this;

        const isError: boolean = error !== "";

        const errorwrapper = isError ? "errorwrapper" : "";

        return html`
            <div>
                <div class="${errorwrapper}">
                    <label class="">
                        <input
                            ?disabled=${this.disabled}
                            type="checkbox"
                            .checked=${checked}
                            @change="${this.onChange}"
                        />
                        <span class=""> ${label} </span>
                    </label>
                </div>
                ${isError ? html`<p class="error">${error}</p>` : nothing}
            </div>
        `;
    }
}
