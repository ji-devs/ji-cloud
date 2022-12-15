import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

const STR_PLAYED = "Plays";
const STR_VIEWED = "Views";
const STR_LIKED = "Likes";
const STR_JI_TEAM = "Ji Team";

type Kind = "jig" | "resource" | "course";

@customElement("asset-card")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    border-radius: 20px;
                    box-shadow: 0 3px 12px 0 rgba(0, 0, 0, 0.12);
                    display: grid;
                    grid-template-rows: 165px auto auto 34px 40px;
                    background-color: #ffffff;
                    width: 280px;
                    height: 288px;
                }
                ::slotted([slot="image"]) {
                    border-radius: 20px 20px 0 0;
                }
                .title {
                    font-size: 22px;
                    font-weight: 600;
                    color: #555555;
                    text-align: center;
                    margin: 0;
                    margin-top: 8px;
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                }
                .played-liked {
                    align-self: flex-start;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    column-gap: 12px;
                }
                .played-liked .count {
                    font-weight: 800;
                }
                .played-liked-divider {
                    width: 2px;
                    height: 16px;
                    background-color: var(--dark-gray-5);
                }
                .ages-language {
                    display: flex;
                    justify-content: space-between;
                    padding: 8px 16px;
                }
                .ages-language,
                .language {
                    display: flex;
                    align-items: center;
                    column-gap: 4px;
                }
                .author-section {
                    font-weight: 500;
                    line-height: 40px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }
                :host([kind=jig]) .author-section,
                :host([kind=course]) .author-section {
                    color: var(--main-blue);
                    border-top: solid 1px var(--light-blue-2);
                }
                :host([kind=resource]) .author-section {
                    background-color: #b4eacb;
                }
                :host([byJiTeam]) .author-section {
                    background-color: var(--light-blue-2);
                }
                .author-section .by-ji-team {
                    font-weight: 800;
                    white-space: pre-wrap;
                }
            `,
        ];
    }

    @property({ reflect: true })
    kind: Kind = "jig";

    @property()
    title: string = "";

    @property({ type: Number })
    playedCount: number = 0;

    @property({ type: Number })
    likedCount: number = 0;

    @property()
    language: string = "";

    @property({ type: Boolean, reflect: true })
    byJiTeam: boolean = false;

    @property()
    author: string = "";

    @property({ type: Boolean, reflect: true })
    flipped: boolean = false;

    renderCount(label: string, count: number) {
        // See related comment in renderCountDivider.
        if (BigInt(count) === BigInt(0)) {
            return nothing;
        }

        return html`
            <div>
                ${label}
                <span class="count">${count}</span>
            </div>
        `;
    }

    renderCountDivider() {
        // There is no guarantee that the value passed into this element is
        // a BigInt, but it _can_ be. Convert the count values so that we're
        // always comparing BigInts.
        // Note: BigInt literals (example 1n) are not available pre-es2020, so
        // we have to use the BigInt() constructor.
        if (BigInt(this.playedCount) === BigInt(0) || BigInt(this.likedCount) === BigInt(0)) {
            return nothing;
        }

        return html`<div class="played-liked-divider"></div>`;
    }

    render() {
        let playedLabel = this.kind === 'resource' ? STR_VIEWED : STR_PLAYED;
        return html`
            <slot name="image"></slot>
            <h3 class="title">${this.title}</h3>
            <div class="played-liked">
                ${this.renderCount(playedLabel, this.playedCount)}
                ${this.renderCountDivider()}
                ${this.renderCount(STR_LIKED, this.likedCount)}
            </div>
            <div class="ages-language">
                <slot name="ages"></slot>
                <div class="language">
                    <img-ui
                        path="entry/home/search-results/language.svg"
                    ></img-ui>
                    <span class="count">${this.language}</span>
                </div>
            </div>
            <div class="author-section">
                ${this.byJiTeam ? html`
                    <img-ui
                        path="entry/home/search-results/ji-logo-blue.svg"
                    ></img-ui>
                    <span class="by-ji-team"
                        >${STR_JI_TEAM} -
                    </span>
                ` : nothing}
                ${this.author}
            </div>
        `;
    }
}
