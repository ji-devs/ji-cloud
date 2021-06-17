import { LitElement, html, css, customElement, property} from "lit-element";
import { mediaUi } from "@utils/path";
import {ThemeKind} from "@elements/_themes/themes";


@customElement("instructions-banner")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {

                position: absolute;
                top: 40px;
                left: 0;
                display: flex;
                justify-content: center;
                width: 100%;
                }
            section {
              padding: 40px;
              border-radius: 32px;
              box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
              background-color: var(--dark-blue-8);
              display: inline-block;
              font-size: 32px;
              font-weight: bold;
              letter-spacing: -0.32px;
              text-align: center;
              color: var(--light-red-4);
            }
            `,
        ];
    }

    @property()
    theme:ThemeKind= "blank";

    render() {
        return html`<section><slot></slot></section>`
    }
}
