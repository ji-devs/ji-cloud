import { LitElement, html, css, customElement } from "lit-element";

@customElement("module-footer")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    justify-content: flex-end;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="btn">
                <slot name="btn"> </slot>
            </div>
        `;
    }
}
