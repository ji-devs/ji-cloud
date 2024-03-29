import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("input-text-underline")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .wrapper {
                    margin-bottom: 16px;
                }
                label {
                    padding-left: 8px;
                }
                span {
                    color: #5590fc;
                    margin-bottom: 8px;
                }
                .input-wrapper {
                    display: flex;
                    align-items: center;
                    border-bottom: solid 1px #e5e7ef;
                    position: relative;
                }

                input {
                    outline: none;
                    border: none;
                    font-size: 16px;
                    padding: 0 8px;
                    width: 100%;
                }
                focus {
                    outline: none;
                }
                ::placeholder {
                    color: #a1a8ad;
                }
            `,
        ];
    }

    onInput(evt: InputEvent) {
        const { value } = evt.target as any;
        this.value = value;

        this.dispatchEvent(
            new CustomEvent("custom-input", {
                detail: { value },
            })
        );
    }
    onChange(evt: InputEvent) {
        const { value } = evt.target as any;
        this.value = value;

        this.dispatchEvent(
            new CustomEvent("custom-change", {
                detail: { value },
            })
        );
    }

    @property()
    label: string = "";

    @property()
    value: string = "";

    @property()
    placeholder: string = "";
    render() {
        const { label, value, placeholder } = this;

        return html`
            <div class="wrapper">
                <label for="name" class="">
                    <span class="">${label}</span>
                    <div class="input-wrapper">
                        <input
                            class=""
                            type="text"
                            .placeholder="${placeholder}"
                            .value="${value}"
                            @input="${this.onInput}"
                            @change="${this.onChange}"
                        />
                    </div>
                </label>
            </div>
        `;
    }
}
