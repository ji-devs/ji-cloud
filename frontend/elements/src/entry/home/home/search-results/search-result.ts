import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

const STR_PLAYED = "Plays";
const STR_LIKED = "Likes";
const STR_JI_TEAM = "Ji Team";
const STR_DESCRIPTION = "Description";
const STR_ADDITIONAL_RESOURCES = "Additional resources";
const STR_SEE_ALL = "See more JIGs by this author";

type Kind = "jig" | "resource";

@customElement("home-search-result")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                    perspective: 2000px;
                }
                .wrapper {
                    display: grid;
                    transition: transform 0.4s;
                    transform-style: preserve-3d;
                    width: 354px;
                    height: 384px;
                    perspective: 1000px;
                }
                :host(:hover) .wrapper {
                    transform: rotateY(180deg);
                }
                .main,
                .hover {
                    width: 100%;
                    height: 100%;
                    /* prefix required for safari https://caniuse.com/?search=backface-visibility */
                    -webkit-backface-visibility: hidden;
                    backface-visibility: hidden;
                    border-radius: 20px;
                    box-shadow: 0 3px 12px 0 rgba(0, 0, 0, 0.12);
                    overflow: hidden;
                }
                .main {
                    grid-column: 1;
                    grid-row: 1;
                    display: grid;
                    grid-template-rows: 200px auto 1fr 34px 40px;
                    height: 100%;
                    row-gap: 8px;
                    background-color: #ffffff;
                }
                .main ::slotted([slot="image"]) {
                    border-radius: 20px 20px 0 0;
                }
                .main .title {
                    font-size: 22px;
                    font-weight: 600;
                    color: #555555;
                    text-align: center;
                    margin: 0;
                    margin-top: 8px;
                }
                .main .played-liked {
                    align-self: flex-start;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    column-gap: 12px;
                }
                .main .played-liked .count {
                    font-weight: 800;
                }
                .main .played-liked-divider {
                    width: 2px;
                    height: 16px;
                    background-color: var(--dark-gray-5);
                }
                .main .ages-language {
                    display: flex;
                    justify-content: space-between;
                    padding: 8px 16px;
                }
                .main .ages-language,
                .main .language {
                    display: flex;
                    align-items: center;
                    column-gap: 4px;
                }
                .main .author-section {
                    font-weight: 500;
                    line-height: 40px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }
                :host([kind=jig]) .main .author-section {
                    color: var(--main-blue);
                    border-top: solid 1px var(--light-blue-2);
                }
                :host([kind=resource]) .main .author-section {
                    background-color: #b4eacb;
                }
                :host([byJiTeam]) .main .author-section {
                    background-color: var(--light-blue-2);
                }
                .main .author-section .by-ji-team {
                    font-weight: 800;
                    white-space: pre-wrap;
                }

                .hover {
                    grid-column: 1;
                    grid-row: 1;
                    height: 100%;
                    color: #ffffff;
                    display: grid;
                    grid-template-rows: 1fr auto;
                    transform: rotateY(180deg);
                }
                :host([kind=jig]) .hover {
                    background-color: var(--dark-blue-2);
                }
                :host([kind=resource]) .hover {
                    background-color: #00844c;
                }
                .hover .scrollable-content {
                    padding: 16px 24px;
                    padding-right: 12px;
                    margin-right: 12px;
                    overflow: auto;
                    scrollbar-width: thin;
                }
                :host([kind=jig]) .hover {
                    scrollbar-color: var(--light-blue-5) transparent;
                }
                :host([kind=resource]) .hover {
                    scrollbar-color: #3f9c6f transparent;
                }
                .hover .scrollable-content::-webkit-scrollbar-track {
                    background-color: transparent;
                    position: absolute;
                }
                .hover .scrollable-content::-webkit-scrollbar {
                    width: 4px;
                }
                .hover .scrollable-content::-webkit-scrollbar-thumb {
                    border-radius: 4px;
                }
                :host([kind=jig]) .scrollable-content::-webkit-scrollbar-thumb {
                    background-color: var(--light-blue-5);
                }
                :host([kind=resource]) .scrollable-content::-webkit-scrollbar-thumb {
                    background-color: #3f9c6f;
                }
                .hover .title {
                    margin: 0;
                    font-size: 16px;
                    font-weight: 600;
                }
                :host([kind=jig]) .hover home-search-result-details:not(:last-child) {
                    border-bottom: solid 1px #3c7df0;
                }
                :host([kind=resource]) .hover home-search-result-details:not(:last-child) {
                    border-bottom: solid 1px #3f9c6f;
                }

                ::slotted(home-search-result-details) {
                    --closed-height: 36px;
                }
                :host([kind=jig]) ::slotted(home-search-result-details) {
                    border-bottom: solid 1px #3c7df0;
                }
                :host([kind=resource]) ::slotted(home-search-result-details) {
                    border-bottom: solid 1px #3f9c6f;
                }
                .hover .additional-resources-items {
                    display: flex;
                    flex-wrap: wrap;
                    gap: 16px;
                    padding: 10px 0;
                }
                .hover ::slotted(a[slot=additional-resources]) {
                    color: #ffffff;
                    text-decoration: none;
                    font-size: 14px;
                    display: flex;
                    column-gap: 6px;
                }
                .hover h3 {
                    font-size: 16px;
                    font-weight: 600;
                    margin: 0;
                }
                .hover .published-at {
                    font-size: 14px;
                    font-weight: 500;
                    margin: 4px 0;
                    display: flex;
                    align-items: center;
                    column-gap: 8px;
                }
                .hover .collapsibles {
                    margin: 20px 0;
                }
                .hover .description {
                    font-size: 14px;
                }
                .hover h4,
                .hover .author-section {
                    font-size: 14px;
                    font-weight: 500;
                    line-height: 38px;
                    margin: 0;
                }
                .hover .author-section {
                    display: flex;
                    justify-content: space-between;
                    flex-wrap: wrap;
                }
                .hover .author-section .left-side {
                    display: flex;
                    align-items: center;
                    column-gap: 6px;
                }
                .hover .play-button-wrapper {
                    height: 82px;
                    box-shadow: 0 3px 12px 0 rgba(0, 0, 0, 0.12);
                    display: grid;
                    place-content: center;
                }
            `,
        ];
    }

    @property({ reflect: true })
    kind: Kind = "jig";

    @property({ type: Boolean })
    new: boolean = false;

    @property({ type: Number })
    leaningPathJigCount?: number = 0;

    @property()
    title: string = "";

    @property({ type: Number })
    playedCount: number = 0;

    @property({ type: Number })
    likedCount: number = 0;

    @property()
    ages: string = "";

    @property()
    language: string = "";

    @property({ type: Boolean, reflect: true })
    byJiTeam: boolean = false;

    @property()
    author: string = "";

    @property()
    publishedAt: string = "";

    @property()
    description: string = "";

    render() {
        return html`
            <div class="wrapper">
                <div class="main">
                    <slot name="image"></slot>
                    <h3 class="title">${this.title}</h3>
                    <div class="played-liked">
                        <div>
                            ${STR_PLAYED}
                            <span class="count">${this.playedCount}</span>
                        </div>
                        <div class="played-liked-divider"></div>
                        <div>
                            ${STR_LIKED}
                            <span class="count">${this.likedCount}</span>
                        </div>
                    </div>
                    <div class="ages-language">
                        <div class="age">
                            <img-ui
                                path="entry/home/search-results/age.svg"
                            ></img-ui>
                            <span class="count">${this.ages}</span>
                        </div>
                        <div class="language">
                            <img-ui
                                path="entry/home/search-results/language.svg"
                            ></img-ui>
                            <span class="count">${this.language}</span>
                        </div>
                    </div>
                    <div class="author-section">
                        ${this.byJiTeam
                            ? html`
                                  <img-ui
                                      path="entry/home/search-results/ji-logo-blue.svg"
                                  ></img-ui>
                                  <span class="by-ji-team"
                                      >${STR_JI_TEAM} -
                                  </span>
                              `
                            : nothing}
                        ${this.author}
                    </div>
                </div>
                <div class="hover">
                    <div class="scrollable-content">
                        <h3 class="title">${this.title}</h3>
                        <p class="published-at">
                            <img-ui
                                path="entry/home/search-results/time.svg"
                            ></img-ui>
                            ${this.publishedAt}
                        </p>
                        <div class="collapsibles">
                            <slot name="categories"></slot>
                            <home-search-result-details>
                                <h4>${STR_DESCRIPTION}</h4>
                                <p class="description">${this.description}</p>
                            </home-search-result-details>
                            ${
                                this.kind === "jig" ? html`
                                    <home-search-result-details>
                                        <h4>${STR_ADDITIONAL_RESOURCES}</h4>
                                        <div class="additional-resources-items">
                                            <slot name="additional-resources"></slot>
                                        </div>
                                    </home-search-result-details>
                                ` : nothing
                            }
                            <div class="author-section">
                                <span class="left-side">
                                    ${this.byJiTeam
                                        ? html`
                                              <img-ui
                                                  path="entry/home/search-results/ji-logo-white.svg"
                                              ></img-ui>
                                              <span class="by-ji-team"
                                                  >${STR_JI_TEAM}
                                              </span>
                                          `
                                        : nothing}
                                    ${this.author}
                                </span>
                                <a>
                                    ${STR_SEE_ALL}
                                    <fa-icon
                                        icon="fa-light fa-chevron-right"
                                    ></fa-icon>
                                </a>
                            </div>
                        </div>
                    </div>
                    <div class="play-button-wrapper">
                        <slot name="play-button"></slot>
                    </div>
                </div>
            </div>
        `;
    }
}
