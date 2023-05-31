import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";
import { nothing } from "lit-html";

const STR_JI_TEAM = "Ji Team";
const STR_LIKED = "Likes";
const STR_PLAYED = "Plays";
const STR_ADDITIONAL_RESOURCES = "Teacher resources";
const STR_PLAYLISTS = "Playlists";
const STR_PLAYLISTS_SUBHEADING = "This JIG is a apart of the following playlists:";

@customElement("jig-play-sidebar-jig-info")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    width: 400px;
                    background-color: #ffffff;
                    font-size: 14px;
                }
                .body {
                    padding: 0 24px;
                }
                section {
                    padding: 20px 0;
                    display: grid;
                    row-gap: 16px;
                }
                section:not(:last-child) {
                    border-bottom: solid 1px #d5e4ff;
                }
                section:first-child {
                    padding-top: 0;
                }
                h4 {
                    margin: 0;
                    color: var(--dark-gray-6);
                    font-size: inherit;
                    font-weight: 500;
                }
                .first-line {
                    display: flex;
                    justify-content: space-between;
                }
                .author {
                    font-weight: 500;
                    line-height: 40px;
                    color: var(--main-blue);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }
                .author .by-ji-team {
                    font-weight: 800;
                    white-space: pre-wrap;
                }
                .published-at {
                    font-size: 12px;
                    font-weight: 500;
                    color: #818b93;
                    display: flex;
                    align-items: center;
                    column-gap: 5px;
                }
                .second-line {
                    display: flex;
                    justify-content: space-between;
                }
                .second-line span {
                    display: flex;
                    align-items: center;
                    column-gap: 5px;
                }
                .description {
                    font-size: 12px;
                    color: var(--dark-gray-6);
                    margin: 0;
                }
                .categories {
                    display: flex;
                    flex-wrap: wrap;
                    grid-gap: 8px;
                }
                .additional-resources-items {
                    display: flex;
                    flex-wrap: wrap;
                    gap: 12px;
                    padding: 8px 0;
                }
                ::slotted(a[slot=additional-resources]) {
                    color: var(--main-blue);
                    font-weight: 500;
                    text-decoration: none;
                    font-size: 12px;
                    display: flex;
                    column-gap: 5px;
                }
                .playlists-section h5 {
                    margin: 0;
                    font-size: 12px;
                    font-weight: normal;
                    color: var(--dark-gray-6);
                }
                .playlists-section .playlists {
                    display: grid;
                }
                .report-section {
                    grid-template-columns: auto auto;
                    align-items: center;
                    justify-content: start;
                    column-gap: 12px;
                }
                .report-section ::slotted([slot="report-sent"]) {
                    color: var(--dark-green-1);
                    font-size: 12px;
                }
            `,
        ];
    }

    @property()
    name: string = "";

    @property({ type: Number })
    playedCount?: number;

    @property({ type: Number })
    likedCount?: number;

    @property()
    language: string = "";

    @property({ type: Boolean })
    byJiTeam: boolean = false;

    @property()
    author: string = "";

    @property()
    publishedAt: string = "";

    @property()
    description: string = "";

    render() {
        return html`
            <popup-body>
                <slot slot="close" name="close"></slot>
                <h3 slot="heading">${this.name}</h3>
                <div class="body" slot="body">
                    <section class="main-info-section">
                        <div class="first-line">
                            <div class="author">
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
                            <span class="published-at">
                                <img-ui
                                    path="entry/home/search-results/time.svg"
                                ></img-ui>
                                ${this.publishedAt}
                            </span>
                        </div>
                        <div class="second-line">
                            <span>
                                <slot name="ages"></slot>
                            </span>
                            <span>
                                <img-ui
                                    path="entry/jig/play/sidebar/language.svg"
                                ></img-ui>
                                ${this.language}
                            </span>
                            <span>
                                ${STR_LIKED}
                                <strong>${this.likedCount}</strong>
                            </span>
                            <span>
                                ${STR_PLAYED}
                                <strong>${this.playedCount}</strong>
                            </span>
                        </div>
                        <p class="description">${this.description}</p>
                        <div class="categories">
                            <slot name="categories"></slot>
                        </div>
                    </section>
                    <section class="additional-resources-section">
                        <h4>${STR_ADDITIONAL_RESOURCES}</h4>
                        <div class="additional-resources-items">
                            <slot name="additional-resources"></slot>
                        </div>
                    </section>
                    <section class="playlists-section">
                        <div>
                            <h4>${STR_PLAYLISTS}</h4>
                            <!-- TODO: enable when ready -->
                            <!-- <h5>${STR_PLAYLISTS_SUBHEADING}</h5> -->
                        </div>
                        <div class="playlists">
                            <!-- TODO: enable when ready -->
                            <!-- <slot name="playlists"></slot> -->
                            Coming soon!
                        </div>
                    </section>
                    <section class="report-section">
                        <slot name="report"></slot>
                        <slot name="report-sent"></slot>
                    </section>
                </div>
            </popup-body>
        `;
    }
}
