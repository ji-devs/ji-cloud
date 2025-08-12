import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/rectangle";
import { homeStyles } from "../../entry/home/home/styles";

const STR_TITLE_JI = "Jewish Interactive";
const STR_TITLE_ABOUT = "About Us";

const STR_JIGS_TITLE = "JIGs";
const STR_JIGS_HEBREW = "Hebrew";
const STR_JIGS_JEWISH_HOLIDAYS = "Jewish holidays";
const STR_JIGS_TORAH = "Torah";
const STR_JIGS_J_STEM = "J-STEM";
const STR_JIGS_ISRAEL = "Israel";
const STR_JIGS_SONGS = "Songs";

const STR_PRODUCT_TITLE = "Product";
const STR_PRODUCT_MANAGE_CLASSROOM = "Manage classroom";
const STR_PRODUCT_CREATE_ACTIVITIES = "Create activities";
const STR_PRODUCT_JOIN_COMMUNITY = "Jigzi community";

const STR_HELP_TITLE = "Help";
const STR_HELP_SUPPORT = "Support & FAQ";
const STR_HELP_TOUR = "Quick tour";
const STR_HELP_TUTORIALS = "Jigzi tutorials";
const STR_HELP_WEBINARS = "Online webinars";
const STR_ACCESSIBILITY_ALERT =
    "We are working toward making JIGZI easy to access for children of all abilities.";

const STR_CONTACT_TITLE = "Contact us";
const STR_CONTACT_EMAIL = "info@jewishinteractive.org";
const STR_CONTACT_TEL = "Tel: ";
const STR_CONTACT_US = "Ji United States";
const STR_CONTACT_UK = "Ji United Kingdom";
const STR_CONTACT_SOUTH_AFRICA = "Ji South Africa";
const STR_CONTACT_ISRAEL = "Ji Israel";

const STR_ABOUT_TITLE = "Who we are";
const STR_ABOUT_TEXT = `Jewish Interactive is a registered 501(c)(3) in the US with tax ID 46-1331618

The Jewish Interactive Educational Trust is a Section 18A (1)(a) in South Africa (Registration IT36/2012) (PBO 930 038 343)

Jewish Interactive is a registered charity in the UK (Charity Number 1151408)`;
const STR_DONATE = "Donate";

const STR_SERVICES_TITLE = "Products & Services";
const STR_SERVICES_TEACHERS = "Teachers";
const STR_SERVICES_PARENTS = "Parents";
const STR_SERVICES_PRIME = "Ji Prime";
const STR_SERVICES_STUDIO = "Ji Studio";
const STR_SERVICES_COLLECTION = "The Ji Collection";
const STR_SERVICES_J_STEM = "J-STEM";

const STR_PRIVACY = "Privacy Policy";
const STR_TERMS = "Terms & Conditions";
const STR_CHILD_PROTECTION = "Child Protection Policy";

const STR_DEDICATION_TEXT = "Dedicated in memory of Yochanan Hillel ben Mordechai (John Hillel Moshal)";

const COMING_SOON_ALERT = "javascript:alert('Coming soon')";

function searchLink(q: string): string {
    return `/home/search?q=${q}`;
}

@customElement("page-footer")
export class _ extends LitElement {
    static get styles() {
        return [
            homeStyles,
            css`
                :host {
                    background-color: var(--dark-blue-8);
                    display: block;
                    color: #ffffff;
                    font-size: 12px;
                    font-weight: 500;
                    padding: 76px 0 24px 0;
                }
                .width-holder {
                    display: grid;
                    grid-template-columns: 3fr auto 2fr;
                    column-gap: 38px;
                }
                .divider {

                    height: 210px;

                    width: 1px;
                    background-color: #63759d;
                    margin-top: 100px;
                }
                h2 {
                    font-size: 36px;
                    font-weight: bold;
                    color: var(--light-blue-6);
                    margin: 0;
                    margin-bottom: 38px;
                }
                ul {
                    list-style: none;
                    padding: 0;
                    margin: 0;
                }
                h4 {
                    font-size: 14px;
                    font-weight: 800;
                    margin: 0;
                }
                dd,
                dl {
                    margin: 0;
                }
                dt {
                    font-weight: 800;
                }
                a {
                    color: inherit;
                    text-decoration: none;
                    line-height: 30px;
                }
                a:hover,
                a:active {
                    text-decoration: underline;
                }
                .columns {
                    display: grid;
                    column-gap: 38px;
                }
                .ji-section .columns {
                    grid-template-columns: repeat(3, auto) min-content;
                }
                .about-section .columns {
                    grid-template-columns: repeat(2, auto);
                }
                .social {
                    display: flex;
                    column-gap: 20px;
                    margin-top: 26px;
                }
                .social img-ui {
                    display: inline-block;
                    background-color: #6ba2fc;
                    height: 32px;
                    width: 32px;
                    border-radius: 50%;
                }
                .social img-ui:hover,
                .social img-ui:active {
                    background-color: #fff;
                }
                .donate-link {
                    display: inline-block;
                    margin-top: 48px;
                }
                .bottom-section {
                    grid-column: 1 / -1;
                    display: flex;
                    align-items: center;
                    column-gap: 16px;
                }
                .bottom-section .kid-safe img {
                    width: 120px;
                }
                .horizontal-slider {
                    width: 87%;
                    margin: 30px auto 22px auto;
                    height: 0;
                    opacity: 0.35;
                    border: solid 1px var(--white);
                }
                .dedication-text {
                    font-family: Poppins;
                    font-size: 14px;
                    font-weight: 500;
                    font-stretch: normal;
                    font-style: normal;
                    line-height: 1.56;
                    letter-spacing: normal;
                    text-align: left;
                    color: var(--light-blue-3);
                }
                .dedication-text-container {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }

                @media (max-width: 1023px) {
                    .width-holder {
                        display: block;
                    }
                    .divider {
                        display: none;
                    }
                    h2 {
                        margin: 20px 0;
                    }
                    .columns {
                        display: block;
                    }
                    .column {
                        margin: 10px 0;
                    }
                }
            `,
        ];
    }

    private accessibilityClicked() {
        alert(STR_ACCESSIBILITY_ALERT);
    }

    render() {
        return html`
            <div class="width-holder">
                <section class="ji-section">
                    <h2>${STR_TITLE_JI}</h2>
                    <div class="columns">
                        <div class="column">
                            <h4>${STR_JIGS_TITLE}</h4>
                            <ul>
                                <li>
                                    <a href="${searchLink(STR_JIGS_HEBREW)}" target="_top">${STR_JIGS_HEBREW}</a>
                                </li>
                                <li>
                                    <a href="${searchLink(
                        STR_JIGS_JEWISH_HOLIDAYS
                    )}" target="_top">${STR_JIGS_JEWISH_HOLIDAYS}</a>
                                </li>
                                <li>
                                    <a href="${searchLink(STR_JIGS_TORAH)}" target="_top">${STR_JIGS_TORAH}</a>
                                </li>
                                <li>
                                    <a href="${searchLink(STR_JIGS_J_STEM)}" target="_top">${STR_JIGS_J_STEM}</a>
                                </li>
                                <li>
                                    <a href="${searchLink(STR_JIGS_ISRAEL)}" target="_top">${STR_JIGS_ISRAEL}</a>
                                </li>
                                <li>
                                    <a href="${searchLink(STR_JIGS_SONGS)}" target="_top">${STR_JIGS_SONGS}</a>
                                </li>
                            </ul>
                        </div>
                        <div class="column">
                            <h4>${STR_PRODUCT_TITLE}</h4>
                            <ul>
                                <li>
                                    <a href="/classroom/codes" target="_top">
                                        ${STR_PRODUCT_MANAGE_CLASSROOM}
                                    </a>
                                </li>
                                <li>
                                    <a href="/asset/edit/studio" target="_top">
                                        ${STR_PRODUCT_CREATE_ACTIVITIES}
                                    </a>
                                </li>
                                <li>
                                    <a href="/community" target="_top">
                                        ${STR_PRODUCT_JOIN_COMMUNITY}
                                    </a>
                                </li>
                            </ul>
                        </div>
                        <div class="column">
                            <h4>${STR_HELP_TITLE}</h4>
                            <ul>
                                <li>
                                    <a href="/home/help" target="_top">${STR_HELP_SUPPORT}</a>
                                </li>
                                <li>
                                    <a href="https://youtu.be/QTqqS_fnfX8" target="_blank">
                                        ${STR_HELP_TOUR}
                                    </a>
                                </li>
                                <li>
                                    <a href="https://www.jewishinteractive.org/how-to-jigzi/" target="_blank">
                                        ${STR_HELP_TUTORIALS}
                                    </a>
                                </li>
                                <li>
                                    <a href="https://www.jewishinteractive.org/jigzi-demo-sessions/" target="_blank">
                                        ${STR_HELP_WEBINARS}
                                    </a>
                                </li>
                            </ul>
                        </div>
                        <div class="column">
                            <h4>${STR_CONTACT_TITLE}</h4>
                            <dl>
                                <dd>
                                    <a href="mailto:${STR_CONTACT_EMAIL}" target="_top">
                                        ${STR_CONTACT_EMAIL}
                                    </a>
                                </dd>
                                <dt>${STR_CONTACT_US}</dt>
                                <dd>
                                    ${STR_CONTACT_TEL}
                                    <a href="tel:+1 866-601-8000" target="_top">
                                        +1 (866) 601-8000
                                    </a>
                                </dd>
                                <dt>${STR_CONTACT_UK}</dt>
                                <dd>
                                    ${STR_CONTACT_TEL}
                                    <a href="tel:+44 (0)79 6641 4417" target="_top">
                                        +44 (0)79 6641 4417
                                    </a>
                                </dd>
                                <dt>${STR_CONTACT_SOUTH_AFRICA}</dt>
                                <dd>
                                    ${STR_CONTACT_TEL}
                                    <a href="tel:+27 (79) 886 5326" target="_top">
                                        +27 (79) 886 5326
                                    </a>
                                </dd>
                                <dt>${STR_CONTACT_ISRAEL}</dt>
                                <dd>
                                    ${STR_CONTACT_TEL}
                                    <a href="tel: +972 (0) 54-597 9555" target="_top">
                                        +972 (0) 54-597 9555
                                    </a>
                                </dd>
                            </dl>
                            <div class="social">
                                <a href="https://www.facebook.com/JewishInteractive" target="_blank">
                                    <img-ui path="entry/home/footer/social-icon-facebook.svg"></img-ui>
                                </a>
                                <a href="https://www.instagram.com/jewishinteractive" target="_blank">
                                    <img-ui path="entry/home/footer/social-icon-instagram.svg"></img-ui>
                                </a>
                                <a href="https://www.youtube.com/user/JewishInteractive" target="_blank">
                                    <img-ui path="entry/home/footer/social-icon-youtube.svg"></img-ui>
                                </a>
                                <a href="https://www.linkedin.com/company/jewish-interactive" target="_blank">
                                    <img-ui path="entry/home/footer/social-icon-linkdin.svg"></img-ui>
                                </a>
                            </div>
                        </div>
                    </div>
                </section>
                <div class="divider"></div>
                <section class="about-section">
                    <h2>${STR_TITLE_ABOUT}</h2>
                    <div class="columns">
                        <div class="column">
                            <h4>${STR_ABOUT_TITLE}</h4>
                            ${STR_ABOUT_TEXT.split("\n").map((text) => {
        return html`<p>${text}</p>`;
        })}
                            <a href="https://www.jewishinteractive.org/donate/" class="donate-link" target="_top">
                                <button-rect color="blue" bold>${STR_DONATE}</button-rect>
                            </a>
                        </div>
                        <div class="column">
                            <h4>${STR_SERVICES_TITLE}</h4>
                            <ul>
                                <li>
                                    <a href="https://www.jewishinteractive.org/ji-teachers/"
                                        target="_blank">${STR_SERVICES_TEACHERS}</a>
                                </li>
                                <li>
                                    <a href="https://www.jewishinteractive.org/ji-parents/"
                                        target="_blank">${STR_SERVICES_PARENTS}</a>
                                </li>
                                <li>
                                    <a href="https://www.jewishinteractive.org/ji-prime-initiatives/"
                                        target="_blank">${STR_SERVICES_PRIME}</a>
                                </li>
                                <li>
                                    <a href="https://www.jewishinteractive.org/ji-studio-5/"
                                        target="_blank">${STR_SERVICES_STUDIO}</a>
                                </li>
                                <li>
                                    <a href="https://www.jewishinteractive.org/ji-products-services/the-ji-collection/"
                                        target="_blank">${STR_SERVICES_COLLECTION}</a>
                                </li>
                                <li>
                                    <a href="https://www.jewishinteractive.org/ji-products-services/jstem/"
                                        target="_blank">${STR_SERVICES_J_STEM}</a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </section>
                <section class="bottom-section">
                    <a href="https://www.jewishinteractive.org/privacy-policy/" target="_blank">${STR_PRIVACY}</a>
                    <span>|</span>
                    <a href="https://www.jewishinteractive.org/terms-and-conditions/" target="_blank">${STR_TERMS}</a>
                    <span>|</span>
                    <a href="https://www.jewishinteractive.org/jewish-interactive-child-protection-policy/"
                        target="_blank">${STR_CHILD_PROTECTION}</a>
                </section>
            </div>
            <div class="horizontal-slider"></div>
            <div class="dedication-text-container"><span class="dedication-text">${STR_DEDICATION_TEXT}</span></div>
        `;
    }
}
