import { LitElement, html, css, customElement, property } from "lit-element";

const STR_MEMBERS = "Members";
const STR_ABOUT = "About";

@customElement("community-badge-details")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                h1, h3, p {
                    margin: 0;
                }
                hr {
                    border: 0;
                    border-top: 1px solid var(--main-yellow);
                    margin: 20px 0;
                    margin: 0;
                }
                :host {
                    display: grid;
                    grid-template-columns: 300px 1fr;
                    gap: 40px;
                }
                .top-section {
                    grid-column: 1 / -1;
                    display: grid;
                    grid-template-columns: 120px auto auto;
                    grid-template-rows: 58px 4px 58px;
                    /* grid-template-rows: auto 1px auto; */
                    height: 120px;
                    column-gap: 32px;
                }
                .top-section ::slotted([slot=img]) {
                    grid-row: 1 / -1;
                    height: 120px;
                    width: 120px;
                    box-sizing: border-box;
                    border-radius: 50%;
                    border: solid 2px var(--main-yellow);
                }
                .top-section h1 {
                    font-size: 40px;
                    font-weight: 800;
                    color: var(--dark-blue-4);
                }
                .top-section .actions {
                    display: flex;
                    gap: 24px;
                    align-items: center;
                    justify-content: end;
                }
                .top-section hr {
                    grid-column: 2 / -1;
                }
                .top-section .member-count {
                    align-self: end;
                    font-size: 18px;
                    font-weight: 600;
                }
                .top-section .member-count b {
                    font-weight: 900;
                }
                .about-section {
                    padding: 40px;
                    border-radius: 16px;
                    border: solid 1px var(--light-orange-3);
                    background-color: var(--white);
                    font-size: 14px;
                    display: grid;
                    row-gap: 16px;
                }
                .about-section h3 {
                    color: var(--dark-blue-4);
                    font-size: 24px;
                    font-weight: bold;
                }
                .members-section {
                    display: grid;
                    align-items: start;
                    row-gap: 24px;
                }
                .members-section .member-heading {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    border-radius: 8px;
                    background-color: #ffecad;
                    padding: 14px 16px;
                }
                .members-section .member-count {
                    color: var(--dark-blue-4);
                    font-size: 24px;
                    font-weight: bold;
                }
                .members-section .member-count span {
                    font-weight: 400;
                }
                .members-section .members {
                    background-color: #ffffff;
                    border-radius: 16px;
                    border: solid 1px var(--light-orange-3);
                }
            `,
        ];
    }

    @property()
    name: string = "";

    @property()
    description: string = "";

    @property({ type: Number })
    memberCount: number = 0;

    render() {
        return html`
            <div class="top-section">
                <slot name="img"></slot>
                <h1>${this.name}</h1>
                <div class="actions">
                    <slot name="actions"></slot>
                </div>
                <hr>
                <p class="member-count">${STR_MEMBERS} <b>${this.memberCount}</b></p>
                <slot name="member-images"></slot>
            </div>
            <div class="about-section">
                <h3>${STR_ABOUT}</h3>
                <p>${this.description}</p>
            </div>
            <div class="members-section">
                <div class="member-heading">
                    <p class="member-count">${STR_MEMBERS} <span>(${this.memberCount})</span></p>
                    <slot name="member-search"></slot>
                </div>
                <div class="members">
                    <slot name="members"></slot>
                </div>
            </div>
        `;
    }
}
