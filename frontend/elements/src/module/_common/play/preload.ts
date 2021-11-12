import {
    LitElement,
    html,
    css,
    customElement,
    property,
    internalProperty,
} from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("module-preload")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: absolute;
                    top: 0;
                    left: 0;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    background-color: rgba(0, 0, 0, 0.5);
                    width: 100%;
                    height: 100%;
                    color: white;
                }
            `,
        ];
    }

    render() {
        return html` <div>Please wait...</div> `;
    }
}
