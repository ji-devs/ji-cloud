import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

@customElement("module-footer")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    margin: 40px 0;
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
