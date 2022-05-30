import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("jig-play-sidebar-report")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                form {
                    display: flex;
                    height: 40px;
                }
                ::slotted(select) {
                    border: solid 2px var(--dark-blue-1);
                    width: 300px;
                    border-radius: 16px 0 0 16px;
                    font-size: 16px;
                    color: var(--dark-gray-6);
                    padding: 0 16px;
                }
                ::slotted(select:focus) {
                    border-color: var(--dark-blue-3);
                    outline: 0;
                }
                ::slotted(button) {
                    background-color: var(--dark-blue-1);
                    color: #ffffff;
                    border: 0;
                    font-size: 16px;
                    font-weight: 600;
                    border-radius: 0 16px 16px 0;
                    cursor: pointer;
                    width: 100px;
                }
                ::slotted(button:active) {
                    background-color: var(--dark-blue-3);
                }
            `,
        ];
    }

    render() {
        return html`
            <form>
                <slot name="select"></slot>
                <slot name="button"></slot>
            </form>
        `;
    }
}
