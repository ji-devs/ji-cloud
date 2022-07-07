import { LitElement, html, css, customElement, property } from "lit-element";

const STR_COMMUNITY = "Community";

@customElement("community-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    min-height: 100vh;
                    min-height: 100dvh;
                    display: grid;
                    grid-template-rows: auto auto 1fr;
                }
                .width-holder {
                    max-width: 1400px;
                    margin: auto;
                }
                header {
                    background-color: #f2777f;
                }
                header .width-holder {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    padding: 40px 30px;
                }
                h2 {
                    margin: 0;
                    color: #fed758;
                    font-size: 48px;
                    font-weight: 900;
                }
                ::slotted([slot=nav]) {
                    display: flex;
                    column-gap: 40px;
                }
                main {
                    padding: 40px 30px;
                    background-color: #fff6d9;
                }
            `,
        ];
    }

    render() {
        return html`
            <slot name="jigzi-header"></slot>
            <header>
                <div class="width-holder">
                    <h2>${STR_COMMUNITY}</h2>
                    <slot name="nav"></slot>
                    <slot name="search-bar"></slot>
                </div>
            </header>
            <main>
                <div class="width-holder">
                    <slot></slot>
                </div>
            </main>
        `;
    }
}
