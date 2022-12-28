import { LitElement, html, css, customElement, property } from "lit-element";

const STR_MEMBERS = "Members";
const STR_ABOUT = "About";

@customElement("community-circle-details")
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
                    margin: 16px 0;
                    margin: 0;
                }
                :host {
                    display: grid;
                    align-items: start;
                    gap: 30px;
                }
                @media (min-width: 1024px) {
                    :host {
                        grid-template-columns: 300px 1fr;
                    }
                }
                .top-section {
                    grid-column: 1 / -1;
                    display: grid;
                    grid-template-columns: 90px auto auto;
                    grid-template-rows: 50px 4px 34px;
                    /* grid-template-rows: auto 1px auto; */
                    height: 90px;
                    column-gap: 24px;
                }
                .top-section .image {
                    grid-row: 1 / -1;
                    height: 90px;
                    width: 90px;
                    box-sizing: border-box;
                    display: inline-grid;
                }
                .top-section .image ::slotted([slot=image]) {
                    height: 100%;
                    width: 100%;
                    border-radius: 50%;
                    display: inline-block;
                    overflow: hidden;
                    /* border: solid 2px var(--main-yellow); */
                    grid-row: 1;
                    grid-column: 1;
                }
                .top-section .image ::slotted([slot=edit-image]) {
                    grid-row: 1;
                    grid-column: 1;
                    justify-self: end;
                    font-size: 12px;
                }
                .top-section header {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: start;
                    column-gap: 8px;
                }
                .top-section header ::slotted(fa-button) {
                    font-size: 12px;
                }
                .top-section h1 {
                    font-size: 30px;
                    font-weight: 800;
                    color: var(--dark-blue-4);
                    display: inline-block;
                }
                .top-section .actions {
                    display: flex;
                    gap: 20px;
                    align-items: center;
                    justify-content: end;
                }
                .top-section hr {
                    grid-column: 2 / -1;
                }
                .top-section .member-count {
                    align-self: end;
                    font-size: 16px;
                    font-weight: 600;
                }
                .top-section .member-count b {
                    font-weight: 900;
                }
                .about-section {
                    padding: 30px;
                    border-radius: 12px;
                    border: solid 1px var(--light-orange-3);
                    background-color: var(--white);
                    font-size: 12px;
                    display: grid;
                    row-gap: 12px;
                }
                .about-section header {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    align-items: center;
                }
                .about-section h3 {
                    color: var(--dark-blue-4);
                    font-size: 20px;
                    font-weight: bold;
                }
                .members-section {
                    display: grid;
                    align-items: start;
                    row-gap: 20px;
                }
                .members-section .member-heading {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    border-radius: 6px;
                    background-color: #ffecad;
                    padding: 12px;
                }
                .members-section .member-count {
                    color: var(--dark-blue-4);
                    font-size: 20px;
                    font-weight: bold;
                }
                .members-section .member-count span {
                    font-weight: 400;
                }
                .members-section .members {
                    display: grid;
                    row-gap: 8px;
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
                <div class="image">
                    <slot name="image"></slot>
                    <slot name="edit-image"></slot>
                </div>
                <header>
                    <h1>${this.name}</h1>
                    <slot name="edit-name"></slot>
                </header>
                <div class="actions">
                    <slot name="actions"></slot>
                </div>
                <hr>
                <p class="member-count">${STR_MEMBERS} <b>${this.memberCount}</b></p>
                <slot name="member-images"></slot>
            </div>
            <div class="about-section">
                <header>
                    <h3>${STR_ABOUT}</h3>
                    <slot name="edit-about"></slot>
                </header>
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
