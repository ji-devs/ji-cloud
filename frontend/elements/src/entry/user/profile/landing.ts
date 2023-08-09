import {
    LitElement,
    html,
    css,
    customElement,
    property,
    query,
} from "lit-element";
import { nothing } from "lit-html";

const STR_MY_SETTINGS = "My Settings";
const STR_BASIC_INFO = "Basic info";
const STR_JIGZI_FILTERS = "Jigzi filters";

const STR_EMAIL = "Email";
const STR_PASSWORD = "Password";
const STR_FIRST_NAME = "First name";
const STR_FAMILY_NAME = "Family name";
const STR_USER_NAME = "Username";
const STR_PREFERRED_LANGUAGE = "Preferred language of communication";
const STR_RELEVANT_AGE_GROUPS = "Relevant age groups";
const STR_RELEVANT_SUBJECTS = "Relevant subjects";
const STR_AFFILIATION = "Affiliation";
const STR_FILTER_MESSAGE = "A note about our filters: Ji believes in making Jewish education accessible to ALL Jews, of all ages and affiliations. If you would like to see only what Jigzi tags as relevant to you, use these filters to fine-tune your search results. If you would like to see ALL our images, resources and JIGs leave these blank.";
const STR_PLAN = "Plan";
const STR_PRICE = "Price";
const STR_AUTO_RENEWS = "Auto renew";
const STR_PAYMENT_METHOD = "Payment method";

@customElement("user-profile")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    background-color: var(--light-blue-1);
                }
                @media (min-width: 1024px) {
                    :host {
                        grid-template-columns: 300px 1fr;
                        height: 100dvh;
                    }
                }
                ::slotted([slot="page-header"]) {
                    grid-column: 1 / -1;
                }
                aside {
                    padding-top: 60px;
                    display: grid;
                    align-content: start;
                    justify-items: center;
                }
                @media (min-width: 1024px) {
                    aside {
                        justify-items: stretch;
                        background-color: var(--light-blue-2);
                        text-align: center;
                    }
                }
                ::slotted(profile-image) {
                    height: 120px;
                    width: 120px;
                    border-radius: 50%;
                    overflow: hidden;
                    margin: 0 auto 16px auto;
                }
                aside ::slotted([slot=profile-image]) .fa-icon {
                    justify-items: end;
                }
                ::slotted(p[slot=reset-password]) {
                    margin: 0;
                    color: var(--dark-green-1);
                    font-weight: 400;
                    font-size: 14px;
                }
                .name {
                    font-size: 17px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                }
                .email-address {
                    font-weight: 500;
                    color: var(--dark-gray-5);
                }
                nav {
                    display: none;
                }
                @media (min-width: 1024px) {
                    nav {
                        margin: 30px;
                        border-top: solid 1px var(--main-blue);
                        border-bottom: solid 1px var(--main-blue);
                        display: grid;
                        row-gap: 18px;
                        padding: 24px 0;
                        text-align: left;
                    }
                    nav a {
                        color: var(--main-blue);
                        text-decoration: none;
                    }
                    nav a.active {
                        /* font-weight: bold; */
                    }

                    main {
                        overflow-y: auto;
                    }
                }
                .main-width-holder {
                    max-width: 950px;
                    margin: 0 auto;
                    padding: 66px 16px;
                    display: grid;
                    row-gap: 36px;
                }
                h1 {
                    color: var(--main-red);
                    font-size: 30px;
                    font-weight: 900;
                    margin: 0;
                    text-align: center;
                }
                @media (min-width: 1024px) {
                    h1 {
                        text-align: left;
                    }
                }
                section {
                    background-color: var(--white);
                    display: grid;
                    row-gap: 40px;
                    column-gap: 10px;
                    border-radius: 8px;
                    padding: 12px;
                }
                @media (min-width: 1024px) {
                    section {
                        border-radius: 24px;
                        padding: 36px;
                        grid-template-columns: minmax(auto, 196px) minmax(auto, 330px) min-content;
                        align-items: center;
                        justify-content: start;
                    }
                }
                h2 {
                    font-size: 20px;
                    font-weight: bold;
                    color: var(--dark-blue-4);
                    margin: 0;
                    margin-bottom: 6px;
                }
                label {
                    font-size: 14px;
                    font-weight: 500;
                    color: #4a4a4a;
                    display: block;
                }

                /* layout */
                @media (max-width: 1023px) {
                    label {
                        display: grid;
                        grid-template-columns: 1fr auto;
                        align-items: center;
                        gap: 8px;
                    }
                    .key {
                        grid-column: 1;
                    }
                    .value::slotted(*),
                    .value {
                        grid-column: 1 / -1;
                    }
                    .edit-button::slotted(*) {
                        grid-row: 1;
                        grid-column: 2;
                    }
                }
                @media (min-width: 1024px) {
                    label {
                        display: contents;
                    }
                    .key {
                        grid-column: 1;
                    }
                    .value::slotted(*),
                    .value {
                        grid-column: 2;
                    }
                    .edit-button::slotted(*) {
                        grid-column: 3;
                    }
                }

                section#basic-info .password-wrapper {
                    display: contents;
                }
                @media (min-width: 1024px) {
                    section#basic-info .password-wrapper {
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                    }
                }
                section#basic-info .password-wrapper .edit-button::slotted(*) {
                    grid-column: 3;
                }
                .filter-message {
                    margin: 0;
                    font-size: 14px;
                    grid-column: 1 / -1;
                }
                .tags-label .tags-wrapper {
                    align-self: stretch;
                    border-radius: 14px;
                    border: solid 1px var(--light-blue-5);
                    padding: 16px;
                    display: flex;
                    flex-wrap: wrap;
                    gap: 12px;
                }
                #plan {
                    row-gap: 16px;
                }
                #plan h2 {
                    margin-bottom: 8px;
                }
                ::slotted([slot=plan-type]),
                ::slotted([slot=plan-price]),
                ::slotted([slot=plan-renews-on]),
                ::slotted([slot=plan-renewal-label]) {
                    font-size: 14px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                    margin: 0;
                }
                ::slotted([slot=plan-auto-renew]) {
                    display: inline-flex;
                    align-items: center;
                    gap: 6px;
                }
                ::slotted([slot=plan-payment-method]) {
                    display: inline-flex;
                    align-items: center;
                    gap: 6px;
                    font-size: 14px;
                    color: var(--dark-gray-6);
                }
                ::slotted([slot=change-to-annual]) {
                    margin-top: 16px;
                    grid-column: 1 / -1;
                    justify-self: start;
                }
                ::slotted(dialog-overlay) {
                    background-color: #00000080;
                }
            `,
        ];
    }

    @property()
    name: string = "";

    @property()
    email: string = "";

    @property({ type: Boolean })
    showPlan: boolean = false;

    @property()
    planSectionTitle: string = "";

    @query("main")
    main!: HTMLElement;

    sections!: NodeListOf<HTMLElement>;
    links!: Map<string, HTMLAnchorElement>;

    private scrollToSection = (e: MouseEvent) => {
        e.preventDefault();
        const id = (e.target as HTMLAnchorElement).getAttribute("href")!;
        this.shadowRoot!.querySelector(id)!.scrollIntoView({
            behavior: "smooth",
        });
    };

    firstUpdated() {
        this.sections = this.shadowRoot!.querySelectorAll("section");

        const links = this.shadowRoot!.querySelectorAll("a[href^='#']");
        const map = new Map();
        for (const a of links) {
            const key = a.getAttribute("href")!.substring(1);
            map.set(key, a);
        }
        this.links = map;

        this.recalculateActive();
    }

    recalculateActive = () => {
        let active = this.sections[0];
        for (const section of this.sections) {
            if (this.main.scrollTop >= section.offsetTop) {
                active = section;
            } else {
                break;
            }
        }

        for (const [id, link] of this.links.entries()) {
            if (id === active.id) link.classList.add("active");
            else link.classList.remove("active");
        }
    };

    render() {
        return html`
            <slot name="page-header"></slot>
            <aside>
                <div slot="profile-image">
                        <slot name="profile-image"><slot name="edit-profile-image"></slot></slot>
                </div>

                <div class="name">${this.name}</div>
                <div class="email-address">${this.email}</div>
                <nav>
                    <a @click="${this.scrollToSection}" href="#basic-info">
                        ${STR_BASIC_INFO}
                    </a>
                    <a @click="${this.scrollToSection}" href="#jigzi-filters">
                        ${STR_JIGZI_FILTERS}
                    </a>
                    ${ this.showPlan ? html`
                        <a @click="${this.scrollToSection}" href="#plan">
                            ${this.planSectionTitle}
                        </a>
                    ` : nothing }
                </nav>
            </aside>
            <main @scroll="${this.recalculateActive}">
                <div class="main-width-holder">
                    <h1>${STR_MY_SETTINGS}</h1>
                    <section id="basic-info">
                        <h2>${STR_BASIC_INFO}</h2>
                        <label>
                            <span class="key">${STR_EMAIL}</span>
                            <slot class="value" name="email"></slot>
                        </label>
                        <label>
                            <span class="key">${STR_PASSWORD}</span>
                            <div class="password-wrapper">
                                <span class="value">************</span>
                                <slot class="edit-button" name="reset-password"></slot>
                            </div>
                        </label>
                        <label>
                            <span class="key">${STR_FIRST_NAME}</span>
                            <slot class="value" name="first-name"></slot>
                        </label>
                        <label>
                            <span class="key">${STR_FAMILY_NAME}</span>
                            <slot class="value" name="family-name"></slot>
                        </label>
                        <label>
                            <span class="key">${STR_USER_NAME}</span>
                            <slot class="value" name="username"></slot>
                        </label>
                        <label>
                            <span class="key">${STR_PREFERRED_LANGUAGE}</span>
                            <slot class="value" name="preferred-language"></slot>
                        </label>
                    </section>
                    <section id="jigzi-filters">
                        <h2>${STR_JIGZI_FILTERS}</h2>
                        <p class="filter-message">${STR_FILTER_MESSAGE}</p>
                        <label class="tags-label">
                            <span class="key">${STR_RELEVANT_AGE_GROUPS}</span>
                            <div class="value tags-wrapper">
                                <slot name="age-groups"></slot>
                            </div>
                            <slot class="edit-button" name="age-groups-edit"></slot>
                        </label>
                        <label class="tags-label">
                            <span class="key">${STR_RELEVANT_SUBJECTS}</span>
                            <div class="value tags-wrapper">
                                <slot name="relevant-subjects"></slot>
                            </div>
                            <slot class="edit-button" name="relevant-subjects-edit"></slot>
                        </label>
                        <label class="tags-label">
                            <span class="key">${STR_AFFILIATION}</span>
                            <div class="value tags-wrapper">
                                <slot name="affiliations"></slot>
                            </div>
                            <slot class="edit-button" name="affiliations-edit"></slot>
                        </label>
                    </section>
                    ${ this.showPlan ? html`
                        <section id="plan">
                            <h2>${this.planSectionTitle}</h2>
                            <label class="tags-label">
                                <span class="key">${STR_PLAN}</span>
                                <div class="value">
                                    <slot name="plan-type"></slot>
                                </div>
                            </label>
                            <label class="tags-label">
                                <span class="key">${STR_PRICE}</span>
                                <div class="value">
                                    <slot name="plan-price"></slot>
                                </div>
                            </label>
                            <label class="tags-label">
                                <span class="key">
                                    <slot name="plan-renewal-label"></slot>
                                </span>
                                <div class="value">
                                    <slot name="plan-renews-on"></slot>
                                </div>
                            </label>
                            <label class="tags-label">
                                <span class="key">${STR_AUTO_RENEWS}</span>
                                <div class="value">
                                    <slot name="plan-auto-renew"></slot>
                                </div>
                            </label>
                            <label class="tags-label">
                                <span class="key">${STR_PAYMENT_METHOD}</span>
                                <div class="value">
                                    <slot name="plan-payment-method"></slot>
                                </div>
                            </label>
                            <slot name="change-to-annual"></slot>
                        </section>
                    ` : nothing }
                </div>
            </main>
            <slot name="popup"></slot>
        `;
    }
}
