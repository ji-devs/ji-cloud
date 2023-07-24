import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("jig-info-category")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                    box-sizing: border-box;
                    padding: 2px 8px;
                    font-size: 11px;
                    border-radius: 16px;
                    border: solid 1px #d6e6fd;
                    color: #2d6ee3;
                    background-color: #d6e6fd;
                    margin: 6px 6px 6px 0px;
                }
            `,
        ];
    }

    @property()
    label: string = "";

    render() {
        return html` ${this.label} `;
    }
}
