import { LitElement, html, css, customElement, property } from 'lit-element';
import '@elements/core/buttons/rectangle';
import { homeStyles } from '../../entry/home/home/styles';

const STR_TITLE_JI = "Jewish Interactive";
const STR_TITLE_ABOUT = "About Us";


const STR_JIGS_TITLE = "JIGs";
const STR_JIGS_HEBREW = "Hebrew JIGs";
const STR_JIGS_JEWISH_HOLIDAYS = "Jewish holidays JIGs";
const STR_JIGS_TORAH = "Torah JIGs";
const STR_JIGS_J_STREAM = "J-Stream";
const STR_JIGS_ISRAEL = "Israel";
const STR_JIGS_SONGS = "Songs";


const STR_PRODUCT_TITLE = "Product";
const STR_PRODUCT_MANAGE = "Manage";
const STR_PRODUCT_CLASSROOM = "Classroom";
const STR_PRODUCT_CREATE_ACTIVITIES = "Create activities";
const STR_PRODUCT_GO_PRO = "Go pro";


const STR_HELP_TITLE = "Help";
const STR_HELP_SUPPORT = "Support & FAQ";
const STR_HELP_TOUR = "Quick tour";
const STR_HELP_TUTORIALS = "JI Tutorials";
const STR_HELP_WEBINARS = "Online webinars";
const STR_HELP_ACCESSIBILITY = "Accessibility";


const STR_CONTACT_TITLE = "Contact us";
const STR_CONTACT_EMAIL = "info@jewishinteractive.org";
const STR_CONTACT_TEL = "Tel: ";
const STR_CONTACT_US = "Ji United States";
const STR_CONTACT_UK = "Ji United Kingdom";
const STR_CONTACT_SOUTH_AFRICA = "Ji South Africa";
const STR_CONTACT_ISRAEL = "Ji Israel";


const STR_ABOUT_TITLE = "Who we are";
const STR_ABOUT_TEXT = `Jewish Interactive is a registered 501(c)(3) in the US with tax ID 46-1331618

The Jewish interactive Educational Trust is a Section 18A (1)(a) in South Africa (Registration IT36/2012) (PBO 930 038 343)

Jewish Interactive is a registered charity in the UK (Charity Number 1151408)`;
const STR_DONATE = "Donate";


const STR_SERVICES_TITLE = "Products & Services";
const STR_SERVICES_TEACHERS = "Teachers";
const STR_SERVICES_PARENTS = "Parents";
const STR_SERVICES_BITES ="JI Bites";
const STR_SERVICES_PRIME = "JI Prime";
const STR_SERVICES_TAP = "JI Tap";
const STR_SERVICES_STUDIO = "JI Studio";
const STR_SERVICES_COLLECTION = "The JI Collection";
const STR_SERVICES_J_STREAM = "J-Stream";
const STR_SERVICES_BLOG = "JI Blog";
const STR_SERVICES_JOBS = "Jobs";


const STR_PRIVACY = "Privacy Policy";
const STR_TERMS = "Terms & Conditions";


@customElement('page-footer')
export class _ extends LitElement {
    static get styles() {
        return [homeStyles, css`
            :host {
                background-color: var(--dark-blue-8);
                display: block;
                color: #ffffff;
                font-size: 14px;
                font-weight: 500;
                padding: 100px 0;
            }
            .width-holder {
                display: grid;
                grid-template-columns: 3fr auto 2fr;
                column-gap: 100px;
            }
            .divider {
                height: 210px;
                width: 1px;
                background-color: #63759d;
                margin-top: 130px;
            }
            h2 {
                font-size: 48px;
                font-weight: bold;
                color: var(--light-blue-6);
                margin: 0;
                margin-bottom: 50px;
            }
            ul {
                list-style: none;
                padding: 0;
                margin: 0;
            }
            h4 {
                font-size: 20px;
                font-weight: 800;
                margin: 0;
            }
            dd, dl {
                margin: 0;
            }
            dt {
                font-weight: 800;
            }
            a {
                color: inherit;
                text-decoration: none;
            }
            a:hover, a:active {
                text-decoration: underline;
            }
            .columns {
                display: grid;
                column-gap: 50px;
            }
            .ji-section .columns {
                grid-template-columns: repeat(3, auto) min-content;
            }
            .about-section .columns {
                grid-template-columns: repeat(2, auto);
            }
            .social {
                display: flex;
                column-gap: 24px;
                margin-top: 32px;
            }
            .social img-ui {
                display: inline-block;
                background-color: #6ba2fc;
                height: 32px;
                width: 32px;
                border-radius: 50%;
            }
            .social img-ui:hover, .social img-ui:active {
                background-color: #fff;
            }
            .donate-link {
                display: inline-block;
                margin-top: 60px;
            }
            .bottom-section {
                grid-column: 1 / -1;
                display: flex;
                align-items: center;
                column-gap: 20px;
            }
        `];
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
                                <li><a href="#">${STR_JIGS_HEBREW}</a></li>
                                <li><a href="#">${STR_JIGS_JEWISH_HOLIDAYS}</a></li>
                                <li><a href="#">${STR_JIGS_TORAH}</a></li>
                                <li><a href="#">${STR_JIGS_J_STREAM}</a></li>
                                <li><a href="#">${STR_JIGS_ISRAEL}</a></li>
                                <li><a href="#">${STR_JIGS_SONGS}</a></li>
                            </ul>
                        </div>
                        <div class="column">
                            <h4>${STR_PRODUCT_TITLE}</h4>
                            <ul>
                                <li><a href="#">${STR_PRODUCT_MANAGE}</a></li>
                                <li><a href="#">${STR_PRODUCT_CLASSROOM}</a></li>
                                <li><a href="#">${STR_PRODUCT_CREATE_ACTIVITIES}</a></li>
                                <li><a href="#">${STR_PRODUCT_GO_PRO}</a></li>
                            </ul>
                        </div>
                        <div class="column">
                            <h4>${STR_HELP_TITLE}</h4>
                            <ul>
                                <li><a href="#">${STR_HELP_SUPPORT}</a></li>
                                <li><a href="#">${STR_HELP_TOUR}</a></li>
                                <li><a href="#">${STR_HELP_TUTORIALS}</a></li>
                                <li><a href="#">${STR_HELP_WEBINARS}</a></li>
                                <li><a href="#">${STR_HELP_ACCESSIBILITY}</a></li>
                            </ul>
                        </div>
                        <div class="column">
                            <h4>${STR_CONTACT_TITLE}</h4>
                            <dl>
                                <dd><a href="mailto:${STR_CONTACT_EMAIL}">${STR_CONTACT_EMAIL}</a></dd>
                                <dt>${STR_CONTACT_US}</dt>
                                <dd>
                                    ${STR_CONTACT_TEL}
                                    <a href="tel:+1-703-517-5182">+1 (703) 517-5182</a>
                                </dd>
                                <dt>${STR_CONTACT_UK}</dt>
                                <dd>
                                    ${STR_CONTACT_TEL}
                                    <a href="tel:+44 (0)79 6641 4417">+44 (0)79 6641 4417</a>
                                </dd>
                                <dt>${STR_CONTACT_SOUTH_AFRICA}</dt>
                                <dd>
                                    ${STR_CONTACT_TEL}
                                    <a href="tel:+27 (79) 886 5326">+27 (79) 886 5326</a>
                                </dd>
                                <dt>${STR_CONTACT_ISRAEL}</dt>
                                <dd>
                                    ${STR_CONTACT_TEL}
                                    <a href="tel: +972 (0) 54-597 9555"> +972 (0) 54-597 9555</a>
                                </dd>
                            </dl>
                            <div class="social">
                                <a href="#">
                                    <img-ui path="entry/home/footer/social-icon-facebook.svg"></img-ui>
                                </a>
                                <a href="#">
                                    <img-ui path="entry/home/footer/social-icon-instagram.svg"></img-ui>
                                </a>
                                <a href="#">
                                    <img-ui path="entry/home/footer/social-icon-youtube.svg"></img-ui>
                                </a>
                                <a href="#">
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
                            ${STR_ABOUT_TEXT.split("\n").map(text => {
                                return html`<p>${text}</p>`;
                            })}
                            <a href="#" class="donate-link">
                                <button-rect color="blue" bold>${STR_DONATE}</button-rect>
                            </a>
                        </div>
                        <div class="column">
                            <h4>${STR_SERVICES_TITLE}</h4>
                            <ul>
                                <li><a href="#">${STR_SERVICES_TEACHERS}</a></li>
                                <li><a href="#">${STR_SERVICES_PARENTS}</a></li>
                                <li><a href="#">${STR_SERVICES_BITES}</a></li>
                                <li><a href="#">${STR_SERVICES_PRIME}</a></li>
                                <li><a href="#">${STR_SERVICES_TAP}</a></li>
                                <li><a href="#">${STR_SERVICES_STUDIO}</a></li>
                                <li><a href="#">${STR_SERVICES_COLLECTION}</a></li>
                                <li><a href="#">${STR_SERVICES_J_STREAM}</a></li>
                                <li><a href="#">${STR_SERVICES_BLOG}</a></li>
                                <li><a href="#">${STR_SERVICES_JOBS}</a></li>
                            </ul>
                        </div>
                    </div>
                </section>
                <section class="bottom-section">
                        <img-ui path="entry/home/footer/kid-safe.png"></img-ui>
                        <a href="#">${STR_PRIVACY}</a>
                        <span>|</span>
                        <a href="#">${STR_TERMS}</a>
                </section>
            </div>
        `;
    }
}
