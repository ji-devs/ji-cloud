import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

@customElement("module-sidebar-body")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    overflow: auto;
                    /* also set height to allow descendants to take full height, feels hacky */
                    height: 100%;
                    min-height: 100%;
                    padding: 0 20px;
                }
                @media (min-width: 1920px) {
                    :host {
                        padding: 0 32px;
                    }
                }

                section {
                    height: 100%;
                    box-sizing: border-box;
                }
            `,
        ];
    }

    render() {
        return html`<section><slot></slot></section>`;
    }
}
