import { html, css, customElement, property, query } from "lit-element";
import "../primitives/base-select";
import { BaseSelect } from "../primitives/base-select";
import "@elements/core/images/ui";
import "../wrapper";

@customElement("input-select")
export class _ extends BaseSelect {
    static get styles() {
        return [
            ...super.styles,
            css`
                .icon {
                    transition: transform .3s;
                }
                :host([open]) .icon {
                    transform: rotate(180deg);
                }
                anchored-overlay::part(overlay) {
                    min-width: 100%;
                    padding-top: 30px;
                    margin-top: -30px;
                    border-radius: 14px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: #fff;
                    z-index: 1;
                }
                :host(:not([nested])) anchored-overlay::part(overlay) {
                    max-height: 250px;
                    overflow: auto;
                }
            `,
        ];
    }

    @property()
    label: string = "";

    @property({ type: Boolean })
    error: boolean = false;

    @property()
    hint: string = "";

    focus() {
        this.open = true;
    }

    render() {
        return html`
            <input-wrapper
                label="${this.label}"
                ?error="${this.error}"
                hint="${this.hint}"
                @click="${this.focus}"
            >
                ${super.render()}
                <img-ui slot="icon" class="icon" path="core/_common/chevron-down-blue.svg"></img-ui>
            </input-wrapper>

        `;
    }
}
