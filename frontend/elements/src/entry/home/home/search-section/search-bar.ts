import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("home-search-bar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: relative;
                    display: flex;
                    font-size: 20px;
                }
                .bar {
                    border-radius: 36px;
                    background-color: #ffffff;
                    width: 100%;
                    display: grid;
                    grid-template-columns: 1fr 200px 200px auto;
                    align-items: center;
                    padding-left: 18px;
                    height: 48px;
                    box-sizing: border-box;
                    border: solid 1px #ffffff;
                }
                .bar:focus-within {
                    border-color: var(--dark-blue-3);
                }
                ::slotted([slot="query"]) {
                    border: none;
                    padding: 0 16px;
                    height: 39px;
                    font-size: 20px;
                    color: var(--dark-gray-6);
                }
                ::slotted([slot="query"])::placeholder {
                    color: var(--light-gray-4);
                }
                ::slotted([slot="query"]:focus) {
                    outline: none;
                }
                ::slotted([slot="age"]),
                ::slotted([slot="language"]) {
                    border-left: solid 2px var(--light-gray-2);
                }
                ::slotted([slot="button"]) {
                    /* cover .bar border */
                    margin: -1px -2px 0 0;
                    height: 48px;
                    font-size: 24px;
                }

                .advanced {
                    position: absolute;
                    /* add 32px for margin */
                    left: calc(100% + 32px);
                    text-align: center;
                }
            `,
        ];
    }

    @property()
    label: string = "";

    render() {
        return html`
            <div class="bar">
                <slot name="query"></slot>
                <slot name="age"></slot>
                <slot name="language"></slot>
                <slot name="button"></slot>
            </div>
            <div class="advanced">
                <slot name="advanced"></slot>
            </div>
        `;
    }
}
