import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

const STR_ABOUT = "About";
const STR_CREATIONS = "Jigzi creations";
const STR_BIO = "Bio";
const STR_BUDGET = "Budget";

@customElement("community-member-details")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                hr {
                    border: 0;
                    border-top: 1px solid #ffe2bf;
                    margin: 20px 0;
                }
                :host {
                    display: grid;
                    grid-template-columns: 300px 1fr;
                    gap: 40px;
                    padding: 40px;
                    background-color: #fff6d9;
                }
                section {
                    padding: 40px;
                    border-radius: 16px;
                    border: solid 1px var(--light-orange-3);
                    background-color: #ffffff;
                }
                section h3 {
                    font-size: 28px;
                    font-weight: bold;
                    color: var(--dark-blue-4);
                    margin: 0;
                }
                .about {
                    font-size: 14px;
                    color: #383838;
                }
                .about .info-line {
                    display: grid;
                    grid-template-columns: 24px 1fr;
                    column-gap: 14px;
                    align-items: center;
                }
                .about .info-line fa-icon {
                    color: var(--main-red);
                    font-size: 22px;
                    text-align: center;
                }
                .creation-tabs {
                    border-bottom: var(--main-blue) 1px solid;
                    margin: 40px 0;
                }
                .creation-assets {
                    display: grid;
                    grid-template-columns: repeat(auto-fill, 230px);
                    gap: 40px;
                }
            `,
        ];
    }

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
            <section class="about">
                <h3>${STR_ABOUT}</h3>
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
                <h3>${STR_BIO}</h3>
                ${
                    this.bio ? html`
                        <p>
                            ${this.bio}
                        </p>
                    ` : nothing
                }
                <hr>
                <h3>${STR_BUDGET}</h3>
            </section>
            <section class="creations">
                <h3>${STR_CREATIONS}</h3>
                <div class="creation-tabs">
                    <slot name="creation-tabs"></slot>
                </div>
                <div class="creation-assets">
                    <slot name="creation-assets"></slot>
                </div>
            </section>
        `;
    }
}
