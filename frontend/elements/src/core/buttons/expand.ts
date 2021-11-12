import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

@customElement("button-expand")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    cursor: pointer;
                }
            `,
        ];
    }

    onToggle() {
        this.expanded = !this.expanded;

        this.dispatchEvent(
            new CustomEvent("custom-toggle", {
                detail: { value: this.expanded },
            })
        );
    }

    @property({ type: Boolean })
    expanded: boolean = false;

    render() {
        const { expanded } = this;

        const icon = expanded ? "expand-all.svg" : "collapse-all.svg";

        const path = `core/buttons/${icon}`;

        return html`<img-ui
            path="${path}"
            @click="${this.onToggle.bind(this)}"
        ></img-ui>`;
    }
}
