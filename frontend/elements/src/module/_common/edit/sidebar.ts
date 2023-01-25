import { LitElement, html, css, customElement } from "lit-element";

@customElement("module-sidebar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    height: 100%;
                    background-color: #f6fafe;
                    width: 300px;
                    grid-template-rows: 110px 1fr;
                }
                .nav {
                    padding-top: 20px;
                }
                .content {
                    display: flex;
                    flex-direction: column;
                    width: 100%;
                    height: 100%;
                    overflow-y: auto;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="nav">
                <slot name="nav"></slot>
            </div>
            <article class="content">
                <slot name="content"></slot>
            </article>
        `;
    }
}
