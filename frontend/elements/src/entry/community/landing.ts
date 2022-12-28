import { LitElement, html, css, customElement } from "lit-element";

const STR_WELCOME_NEW_MEMBERS = "Welcome new members";
const STR_NEW_CIRCLES = "Join a new circle";

@customElement("community-landing")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: auto auto;
                    align-items: start;
                    gap: 30px;
                }
                section {
                    padding: 28px;
                    border-radius: 14px;
                    border: solid 1px var(--light-orange-3);
                    background-color: #ffffff;
                    display: grid;
                    row-gap: 24px;
                }
                h3 {
                    font-size: 20px;
                    font-weight: bold;
                    color: var(--dark-blue-4);
                    margin: 0;
                }
                h3 fa-icon {
                    color: var(--main-yellow);
                    margin-right: 4px;
                }
                ::slotted([slot=members]) {
                    text-decoration: none;
                    display: grid;
                    grid-template-columns: 50px 1fr;
                    grid-auto-rows: 50px;
                    column-gap: 30px;
                    align-items: center;
                    border-top: solid 1px var(--main-yellow);
                    padding: 12px 0;
                    color: var(--dark-gray-6);
                    font-size: 14px;
                    font-weight: 500;
                }
                ::slotted([slot=members-link]) {
                    justify-self: center;
                }
                .circle-items {
                    border-top: solid 1px var(--main-yellow);
                    padding: 12px 0;
                    display: grid;
                    grid-template-columns: repeat(3, auto);
                    justify-content: space-between;
                    row-gap: 24px;
                }
                ::slotted([slot=circles]) {
                    height: 96px;
                    width: 70px;
                    display: grid;
                    justify-items: center;
                    grid-template-rows: 70px 16px;
                    line-height: 16px;
                    font-size: 13px;
                    text-decoration: none;
                    color: var(--dark-gray-6);
                    align-content: space-between;
                }
                ::slotted([slot=circles-link]) {
                    justify-self: center;
                }
            `,
        ];
    }

    render() {
        return html`
            <section class="member-section">
                <h3>
                    <fa-icon icon="fa-solid fa-stars"></fa-icon>
                    ${STR_WELCOME_NEW_MEMBERS}
                </h3>
                <div class="member-items">
                    <slot name="members"></slot>
                </div>
                <slot name="members-link"></slot>
            </section>
            <section class="circle-section">
                <h3>
                    <fa-icon icon="fa-solid fa-people-group"></fa-icon>
                    ${STR_NEW_CIRCLES}
                </h3>
                <div class="circle-items">
                    <slot name="circles"></slot>
                </div>
                <slot name="circles-link"></slot>
            </section>
        `;
    }
}
