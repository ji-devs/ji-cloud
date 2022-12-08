import { LitElement, html, css, customElement } from "lit-element";

@customElement("menu-tabs")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    /*
                    using minmax(0, 1fr) instead of just 1fr to allow the items inside to overflow without growing the container.
                    https://stackoverflow.com/a/52861514/5253155
                    https://stackoverflow.com/a/43312314/5253155
                */
                    grid-template-rows: 40px minmax(0, 1fr);
                    height: 100%;
                }
                .tabs-wrapper {
                    display: flex;
                }
                .body {
                    background-color: #e9eff8;
                    padding-top: 20px;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="tabs-wrapper">
                <slot name="tabs"></slot>
            </div>
            <div class="body">
                <slot name="body"></slot>
            </div>
        `;
    }
}
