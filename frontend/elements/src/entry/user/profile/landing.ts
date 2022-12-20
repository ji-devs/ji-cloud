import {
    LitElement,
    html,
    css,
    customElement,
    property,
    query,
} from "lit-element";

const STR_BASIC_INFO = "Basic info";
const STR_JIGZI_FILTERS = "Jigzi Filters";
const STR_MY_SETTINGS = "My Settings";

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

@customElement("user-profile")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-rows: auto 1fr;
                    grid-template-columns: 400px 1fr;
                    height: 100vh;
                }
                ::slotted([slot="page-header"]) {
                    grid-column: 1 / -1;
                }
                aside {
                    background-color: var(--light-blue-2);
                    padding-top: 80px;
                    text-align: center;
                }
                aside ::slotted([slot=profile-image]) {
                    display: inline-grid;
                    height: 156px;
                    width: 156px;
                    border-radius: 50%;
                    margin: 0 auto 20px auto;
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
                    font-size: 22px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                }
                .email-address {
                    font-weight: 500;
                    color: var(--dark-gray-5);
                }
                nav {
                    margin: 40px;
                    border-top: solid 1px var(--main-blue);
                    border-bottom: solid 1px var(--main-blue);
                    display: grid;
                    row-gap: 24px;
                    padding: 32px 0;
                    text-align: left;
                }
                nav a {
                    color: var(--main-blue);
                    text-decoration: none;
                }
                /* nav a.active {
                    /* font-weight: bold; */
                /* } */ 
                main {
                    background-color: var(--light-blue-1);
                    overflow-y: auto;
                }
                .main-width-holder {
                    max-width: 1060px;
                    margin: 0 auto;
                    padding: 88px 20px;
                    display: grid;
                    row-gap: 48px;
                }
                h1 {
                    color: var(--main-red);
                    font-size: 40px;
                    font-weight: 900;
                    margin: 0;
                }
                section {
                    border-radius: 24px;
                    background-color: var(--white);
                    padding: 48px;
                    display: grid;
                    row-gap: 40px;
                }
                h2 {
                    font-size: 24px;
                    font-weight: bold;
                    color: var(--dark-blue-4);
                    margin: 0;
                    margin-bottom: 8px;
                }
                label {
                    font-size: 16px;
                    font-weight: 500;
                    color: #4a4a4a;
                    display: grid;
                    grid-template-columns: 248px 440px auto;
                    align-items: center;
                    justify-content: start;
                }
                section#basic-info {
                    grid-template-columns: auto auto;
                }
                section#basic-info .left-side {
                    display: grid;
                    row-gap: 40px;
                }
                section#basic-info .password-wrapper {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }
                .tags-label .tags-wrapper {
                    border-radius: 14px;
                    border: solid 1px var(--light-blue-5);
                    padding: 20px;
                    display: flex;
                    flex-wrap: wrap;
                    gap: 14px;
                }
                .tags-label ::slotted(button-rect) {
                    margin-left: 40px;
                }
                .profile-image-wrapper {
                    display: grid;
                    justify-items: center;
                    row-gap: 16px;
                    width: 156px;
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
                    <a @click="${this.scrollToSection}" href="#basic-info"
                        >${STR_BASIC_INFO}</a
                    >
                    <a
                        @click="${this.scrollToSection}"
                        href="#jigzi-filters"
                        >${STR_JIGZI_FILTERS}</a
                    >
                </nav>
            </aside>
            <main @scroll="${this.recalculateActive}">
                <div class="main-width-holder">
                    <h1>${STR_MY_SETTINGS}</h1>
                    <section id="basic-info">
                        <div class="left-side">
                            <h2>${STR_BASIC_INFO}</h2>
                            <label>
                                <span>${STR_EMAIL}</span>
                                <slot name="email"></slot>
                            </label>
                            <label>
                                <span>${STR_PASSWORD}</span>
                                <div class="password-wrapper">
                                    <span>************</span>
                                    <slot name="reset-password"></slot>
                                </div>
                            </label>
                            <label>
                                <span>${STR_FIRST_NAME}</span>
                                <slot name="first-name"></slot>
                            </label>
                            <label>
                                <span>${STR_FAMILY_NAME}</span>
                                <slot name="family-name"></slot>
                            </label>
                            <label>
                                <span>${STR_USER_NAME}</span>
                                <slot name="username"></slot>
                            </label>
                            <label>
                                <span>${STR_PREFERRED_LANGUAGE}</span>
                                <slot name="preferred-language"></slot>
                            </label>
                        </div>
                    </section>
                    <section id="jigzi-filters">
                        <h2>${STR_JIGZI_FILTERS}</h2>
                        <p>${STR_FILTER_MESSAGE}</p>
                        <label class="tags-label">
                            <span>${STR_RELEVANT_AGE_GROUPS}</span>
                            <div class="tags-wrapper">
                                <slot name="age-groups"></slot>
                            </div>
                            <slot name="age-groups-edit"></slot>
                        </label>
                        <label class="tags-label">
                            <span>${STR_RELEVANT_SUBJECTS}</span>
                            <div class="tags-wrapper">
                                <slot name="relevant-subjects"></slot>
                            </div>
                            <slot name="relevant-subjects-edit"></slot>
                        </label>
                        <label class="tags-label">
                            <span>${STR_AFFILIATION}</span>
                            <div class="tags-wrapper">
                                <slot name="affiliations"></slot>
                            </div>
                            <slot name="affiliations-edit"></slot>
                        </label>
                    </section>
                </div>
            </main>
            <slot name="popup"></slot>
        `;
    }
}
