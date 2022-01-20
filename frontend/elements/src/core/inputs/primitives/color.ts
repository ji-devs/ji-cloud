import {
    LitElement,
    html,
    css,
    customElement,
    property,
    query,
} from "lit-element";

@customElement("input-color")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                    justify-content: start;
                }
                ::slotted(*) {
                    grid-column: 1;
                    grid-row: 1;
                }
                input {
                    grid-column: 1;
                    grid-row: 1;
                    /* not using display none intentionally */
                    visibility: hidden;
                    height: 0.1px;
                    padding: 0;
                    margin: 0;
                }
            `,
        ];
    }

    @property()
    value: string = "#ffffff";

    @query("input[type=color]")
    input!: HTMLInputElement;

    private open() {
        this.input.focus(); // read online that this is required for safari to open the color picker, havn't varified it
        this.input.click();
    }

    private onChange(evt: InputEvent) {
        const { value } = evt.target as any;
        this.value = value;

        this.dispatchEvent(
            new CustomEvent("custom-change", {
                detail: { value },
            })
        );
    }

    render() {
        return html`
            <input
                type="color"
                .value="${this.value}"
                @change="${this.onChange}"
            />
            <slot @click="${() => this.open()}" slot="trigger"></slot>
        `;
    }
}
