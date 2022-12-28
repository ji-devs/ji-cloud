import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

const STR_ABOUT = "About";
const STR_CREATIONS = "'s creations";
const STR_NETWORK = "'s network";
const STR_BIO = "Bio";
const STR_CIRCLE = "Circles";

@customElement("community-member-details")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                hr {
                    border: 0;
                    border-top: 1px solid #ffe2bf;
                    margin: 16px 0;
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
                .top {
                    grid-column: 1 / -1;
                    display: grid;
                    grid-template-columns: 90px 1fr auto;
                    align-items: start;
                    column-gap: 24px;
                }
                .top .profile-image {
                    display: inline-grid;
                    height: 90px;
                    width: 90px;
                }
                .top .profile-image ::slotted([slot=profile-image]) {
                    height: 100%;
                    width: 100%;
                    border-radius: 50%;
                    overflow: hidden;
                    grid-row: 1;
                    grid-column: 1;
                }
                .top .profile-image ::slotted([slot=edit-profile-image]) {
                    grid-row: 1;
                    grid-column: 1;
                    justify-self: end;
                    font-size: 12px;
                }
                .top h1 {
                    font-size: 30px;
                    font-weight: 800;
                    color: var(--dark-blue-4);
                    margin: 0;
                }
                section {
                    padding: 24px;
                    border-radius: 12px;
                    border: solid 1px var(--light-orange-3);
                    background-color: #ffffff;
                }
                section h3 {
                    font-size: 20px;
                    font-weight: bold;
                    color: var(--dark-blue-4);
                    margin: 0;
                }
                .about {
                    font-size: 12px;
                    color: #383838;
                }
                .about header {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    align-items: center;
                }
                .about .info-line {
                    display: grid;
                    grid-template-columns: 20px 1fr;
                    column-gap: 12px;
                    align-items: center;
                }
                .about .info-line fa-icon {
                    color: var(--main-red);
                    font-size: 18px;
                    text-align: center;
                }
                .circles-wrapper {
                    display: flex;
                    flex-wrap: wrap;
                    gap: 20px;
                    justify-content: space-between;
                }
                .circles-wrapper ::slotted([slot=circles]) {
                    height: 90px;
                    width: 70px;
                    display: grid;
                    justify-items: center;
                    grid-template-rows: 70px 16px;
                    line-height: 16px;
                    font-size: 12px;
                    text-decoration: none;
                    color: var(--dark-gray-6);
                    align-content: space-between;
                }
                .right-sections {
                    display: grid;
                    gap: inherit;
                }
                .creation-tabs, .connection-tabs {
                    border-bottom: var(--main-blue) 1px solid;
                    margin: 30px 0;
                }
                .creation-assets {
                    display: flex;
                    flex-wrap: wrap;
                    gap: 30px;
                    justify-content: center;
                }
            `,
        ];
    }

    @property()
    givenName: string = "";

    @property()
    familyName: string = "";

    @property()
    city?: string;

    @property()
    organization?: string;

    @property()
    persona?: string;

    @property()
    language?: string;

    @property()
    bio?: string;

    render() {
        return html`
            <div class="top">
                <div class="profile-image">
                    <slot name="profile-image"></slot>
                    <slot name="edit-profile-image"></slot>
                </div>
                <h1>${this.givenName} ${this.familyName}</h1>
                <slot name="follow"></slot>
            </div>
            <section class="about">
                <header>
                    <h3>${STR_ABOUT}</h3>
                    <slot name="edit-about"></slot>
                </header>
                ${
                    this.city ? html`
                        <p class="info-line">
                            <fa-icon icon="fa-solid fa-location-dot"></fa-icon>
                            ${this.city}
                        </p>
                    ` : nothing
                }
                ${
                    this.organization ? html`
                        <p class="info-line">
                            <fa-icon icon="fa-solid fa-briefcase"></fa-icon>
                            ${this.organization}
                        </p>
                    ` : nothing
                }
                ${
                    this.persona ? html`
                        <p class="info-line">
                            <fa-icon icon="fa-regular fa-id-card-clip"></fa-icon>
                            ${this.persona}
                        </p>
                    ` : nothing
                }
                ${
                    this.language ? html`
                        <p class="info-line">
                            <fa-icon icon="fa-solid fa-globe"></fa-icon>
                            ${this.language}
                        </p>
                    ` : nothing
                }
                <hr>
                <header>
                    <h3>${STR_BIO}</h3>
                    <slot name="edit-bio"></slot>
                </header>
                ${
                    this.bio ? html`
                        <p>
                            ${this.bio}
                        </p>
                    ` : nothing
                }
                <hr>
                <h3>${STR_CIRCLE}</h3>
                <div class="circles-wrapper">
                    <slot name="circles"></slot>
                </div>
            </section>
            <div class="right-sections">
                <section class="creations">
                    <h3>${this.givenName} ${this.familyName}${STR_CREATIONS}</h3>
                    <div class="creation-tabs">
                        <slot name="creation-tabs"></slot>
                    </div>
                    <div class="creation-assets">
                        <slot name="creation-assets"></slot>
                    </div>
                </section>
                <section class="connections">
                    <h3>${this.givenName} ${this.familyName}${STR_NETWORK}</h3>
                    <div class="connection-tabs">
                        <slot name="connection-tabs"></slot>
                    </div>
                    <div class="connection-members">
                        <slot name="connection-members"></slot>
                    </div>
                </section>
            </div>
        `;
    }
}
