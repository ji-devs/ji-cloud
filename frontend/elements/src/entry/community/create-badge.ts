import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";

const STR_CREATE = "Create a new badge";

@customElement("community-create-badge")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .overlay {
                    position: fixed;
                    top: 0;
                    left: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    width: 100vw;
                    height: 100vh;
                    opacity: 0.8;
                    background-color: var(--light-blue-3);
                    z-index: 10;
                }
                popup-body {
                    position: fixed;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    z-index: 10;
                    

                    border-radius: 16px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                }
                .body {
                    padding: 0 30px 30px 30px;
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    grid-template-areas: 
                        "name        logo"
                        "description logo"
                        "submit      submit";
                    grid-gap: 40px;
                }
                ::slotted([slot=close]) {
                    font-size: 14px;
                }
                ::slotted([slot=name]) {
                    grid-area: name;
                }
                ::slotted([slot=description]) {
                    grid-area: description;
                }
                ::slotted([slot=logo]) {
                    grid-area: logo;
                }
                ::slotted([slot=submit]) {
                    grid-area: submit;
                    justify-self: center;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="overlay"></div>
            <popup-body>
                <slot slot="close" name="close"></slot>
                <h3 slot="heading">${STR_CREATE}</h3>
                <div slot="body" class="body">
                    <slot name="name"></slot>
                    <slot name="description"></slot>
                    <slot name="logo"></slot>
                    <slot name="submit"></slot>
                </div>
            </popup-body>
        `;
    }
}