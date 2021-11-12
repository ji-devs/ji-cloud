import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

const STR_CONTINUE = "Continue";

@customElement("module-footer-continue-button")
export class _ extends LitElement {
    static get styles() {
        return [css``];
    }

    @property({ type: Boolean })
    enabled: boolean = false;

    render() {
        const { enabled } = this;

        const pointer = enabled ? "initial" : "none";

        return html`
            <button-rect
                .disabled=${!enabled}
                style="pointer-events: ${pointer}"
                size="small"
                iconAfter="arrow"
                @click=${() => {
                    if (enabled) {
                        this.dispatchEvent(new Event("next"));
                    }
                }}
                >${STR_CONTINUE}
            </button-rect>
        `;
    }
}
