import { LitElement, html, css, customElement, property } from "lit-element";
import { homeStyles } from "../styles";

@customElement("home-quick-search")
export class _ extends LitElement {
    static get styles() {
        return [
            homeStyles,
            css`
                :host {
                    background-color: var(--light-blue-3);
                    display: block;
                }
                .width-holder {
                    display: flex;
                    justify-content: space-between;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="width-holder">
                <slot></slot>
            </div>
        `;
    }
}
