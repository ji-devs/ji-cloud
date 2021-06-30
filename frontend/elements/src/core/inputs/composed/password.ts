import { LitElement, html, css, customElement, property } from "lit-element";
import "../wrapper";

@customElement("input-password")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            `,
        ];
    }

    @property()
    label: string = "";

    @property()
    value: string = "";

    @property({ type: Boolean })
    error: boolean = false;

    @property()
    placeholder: string = "";

    @property()
    hint: string = "";

    @property({ type: Boolean })
    visible: boolean = false;

    toggleVisible() {
        this.visible = !this.visible;
    }

    onInput(evt:InputEvent) {
        const {value} = (evt.target as any);
        this.value = value;

        this.dispatchEvent(new CustomEvent("custom-input", {
            detail: { value },
        }))
    }
    onChange(evt:InputEvent) {
        const {value} = (evt.target as any);
        this.value = value;

        this.dispatchEvent(new CustomEvent("custom-change", {
            detail: { value },
        }))
    }

    render() {
        const path = `core/inputs/eye-${this.visible ? "open" : "closed"}.svg`;

        const inputType = this.visible ? "text" : "password";

        return html`
            <input-wrapper
                label="${this.label}"
                ?error="${this.error}"
                hint="${this.hint}"
            >
                <input
                    placeholder="${this.placeholder}"
                    value="${this.value}"
                    type="${inputType}"
                >
                <img-ui @click="${this.toggleVisible}" slot="icon" path="${path}"></img-ui>
            </input-wrapper>

        `;
    }
}
