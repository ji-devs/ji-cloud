import { LitElement, html, css, customElement } from "lit-element";
import "@elements/core/titles/variants/underlined-title";

@customElement("admin-export")
export class _ extends LitElement {
    static styles = [
        css`
            :host {
                padding: 40px;
                margin-top: 40px;
            }
            .wrapper {
                display: grid;
                grid-template-columns: 300px;
                padding-top: 40px;
                grid-gap: 20px;
            }
        `
    ];

    render() {
        return html`
            <underlined-title title="Export data"></underlined-title>
            <div class="wrapper">
                <slot></slot>
            </div>
        `;
    }
}
