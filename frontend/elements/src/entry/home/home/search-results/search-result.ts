import { LitElement, html, css, customElement, property, query } from "lit-element";
import { nothing } from "lit-html";
import { CarouselSingle } from "@elements/core/carousel/single";

const STR_JI_TEAM = "Ji Team";
const STR_DESCRIPTION = "Description";
const STR_ADDITIONAL_RESOURCES = "Teacher resources";
const STR_SEE_ALL = "See more JIGs by this author";

type Kind = "jig" | "resource" | "playlist";

@customElement("home-search-result")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                @keyframes jump {
                    0% {
                        transform: scaleX(-1) translateY(0px);
                    }
                    20% {
                        transform: scaleX(-1) translateY(0px);
                    }
                    36% {
                        transform: scaleX(-1) translateY(-78px);
                    }
                    52% {
                        transform: scaleX(-1) translateY(-55px);
                    }
                    70% {
                        transform: scaleX(-1) translateY(-64px);
                    }
                    90% {
                        transform: scaleX(-1) translateY(-58px);
                    }
                    100% {
                        transform: scaleX(-1) translateY(-60px);
                    }
                }

                :host {
                    display: inline-block;
                    perspective: 2000px;
                }
                /* --line-color is used for category borders and lines */
                :host([kind=jig]) {
                    --line-color: var(--light-orange-3);
                }
                :host([kind=resource]) {
                    --line-color: #beedfe;
                }
                :host([kind=playlist]) {
                    --line-color: var(--green-3);
                }
                .wrapper {
                    display: grid;
                    transition: transform 0.4s;
                    transform-style: preserve-3d;
                    width: 280px;
                    height: 280px;
                    perspective: 1000px;
                    position: relative;
                }
                :host(:hover) .wrapper, :host([flipped]) .wrapper {
                /* .wrapper, :host([flipped]) .wrapper { */
                    transform: rotateY(180deg);
                    /* Safari wont register clicks (on share and like buttons, so left side) if rotated >= 180deg. Don't know why. Remove once Safari is fixed */
                    transform: rotateY(179deg);
                }
                ::slotted([slot=front]) {
                    z-index: 2;
                    /* safari seems to require backface-visibility here */
                    backface-visibility: hidden;
                }
                .back {
                    width: 100%;
                    height: 100%;
                    backface-visibility: hidden;
                    border-radius: 20px;
                    box-shadow: 0 3px 12px 0 rgba(0, 0, 0, 0.12);
                    overflow: hidden;
                    position: absolute;
                    grid-column: 1;
                    grid-row: 1;
                    display: grid;
                    grid-template-rows: 1fr auto;
                    transform: rotateY(180deg);
                    z-index: 3;
                }
                :host([kind=jig]) .back {
                    background-color: var(--light-orange-1);
                }
                :host([kind=resource]) .back {
                    background-color: #e3f5fd;
                }
                :host([kind=playlist]) .back {
                    background-color: #e9fae5;
                }
                .back .scrollable-content {
                    padding: 16px 5px 16px 16px;
                    margin-right: 5px;
                    overflow: auto;
                    scrollbar-width: thin;
                    display: grid;
                    /* row-gap: 10px; */
                    align-content: start;
                }
                :host([kind=jig]) .back,
                :host([kind=playlist]) .back {
                    scrollbar-color: var(--light-gray-2) transparent;
                }
                :host([kind=resource]) .back {
                    scrollbar-color: var(--light-gray-2) transparent;
                }
                .back .scrollable-content::-webkit-scrollbar-track {
                    background-color: transparent;
                    position: absolute;
                    margin: 15px;
                }
                .back .scrollable-content::-webkit-scrollbar {
                    width: 6px;
                }
                .back .scrollable-content::-webkit-scrollbar-thumb {
                    border-radius: 3px;
                }
                .scrollable-content::-webkit-scrollbar-thumb {
                    background-color: var(--light-gray-2);
                }
                .thumbnails {
                    display: grid;
                    grid-template-columns: auto auto auto;
                    justify-content: space-between;
                    align-items: center;
                }
                .thumbnails fa-button {
                    color: var(--dark-gray-6);
                }
                .thumbnails carousel-single {
                    aspect-ratio: 16/9;
                    height: 110px;
                    /* safari need this to be grid, no idea why */
                    display: grid;
                }
                .thumbnails carousel-single ::slotted([slot=thumbnails]) {
                    border-radius: 8px;
                    margin: 1px;
                }
                .back .name {
                    margin: 10px 0;
                    font-size: 13px;
                    font-weight: 600;
                    color: var(--dark-blue-4);
                }
                .back .published-at {
                    font-size: 13px;
                    font-weight: 300;
                    color: var(--dark-gray-5);
                    display: flex;
                    align-items: center;
                    column-gap: 6px;
                }
                ::slotted(home-search-result-details),
                .back home-search-result-details:not(:last-child) {
                    border-bottom: solid 1px var(--line-color);
                }
                ::slotted(home-search-result-details) {
                    --closed-height: 36px;
                }
                .back .additional-resources-items {
                    display: flex;
                    flex-wrap: wrap;
                    gap: 16px;
                    padding: 10px 0;
                }
                .back ::slotted(a[slot=additional-resources]) {
                    color: var(--dark-gray-5);
                    text-decoration: none;
                    font-size: 14px;
                    display: flex;
                    column-gap: 6px;
                }
                .back h3 {
                    font-size: 16px;
                    font-weight: 600;
                    margin: 0;
                }
                .back .description {
                    font-size: 13px;
                    font-weight: 300;
                    word-break: break-word;
                    white-space: pre-wrap;
                }
                .back h4,
                .back .author-section {
                    font-size: 13px;
                    font-weight: 500;
                }
                .back .author-section {
                    display: flex;
                    justify-content: space-between;
                    flex-wrap: wrap;
                    padding: 10px 0;
                }
                .back .author-section .left-side {
                    display: flex;
                    align-items: center;
                    column-gap: 6px;
                    color: inherit;
                    text-decoration: none;
                }
                .back .author-section .left-side:hover {
                    color: var(--main-blue);
                }
                .back .result-actions {
                    height: 40px;
                    box-shadow: 0 3px 12px 0 rgba(0, 0, 0, 0.12);
                    display: grid;
                    place-content: center;
                    display: grid;
                    grid-template-columns: 50% 50%;
                    padding: 0 16px;
                }
                .back .extra-actions {
                    display: flex;
                    align-items: center;
                }
                .back .extra-actions ::slotted([slot=actions]) {
                    text-decoration: none;
                    font-size: 16px;
                    height: 32px;
                    width: 32px;
                    display: inline-grid;
                    place-content: center;
                    color: var(--dark-blue-1);
                }
                .back .extra-actions ::slotted([slot=actions]:hover) {
                    color: var(--dark-blue-2);
                }
                .back .primary-action {
                    margin-left: auto;
                    align-self: center;
                }

                img-ui.jiggling {
                    position: absolute;
                    top: 0%;
                    left: 5%;
                    z-index: 1;
                }

                :host(:hover) img-ui.jiggling {
                    animation: jump 1s ease-in-out;
                    transform: scaleX(-1) translateY(-60px);
                }

                .premium {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    position: absolute;
                    width: 100%;
                    top: -26px;
                    pointer-events: none;
                }

                /*
                * [Ty] Added to temporarily disable the jiggling on the back of search result
                * cards
                */
                @media not all and (min-resolution:.001dpcm) {
                    @supports (-webkit-appearance:none) {
                        img-ui.jiggling {
                            display: none;
                        }
                    }
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
    name: string = "";

    @property({ type: Number })
    playedCount: number = 0;

    @property({ type: Number })
    likedCount: number = 0;

    @property()
    language: string = "";

    @property({ type: Boolean, reflect: true })
    byJiTeam: boolean = false;

    @property()
    authorName: string = "";

    @property()
    authorLink: string = "";

    @property()
    publishedAt: string = "";

    @property()
    description: string = "";

    @property({ type: Boolean })
    showAdditionalResources: boolean = true;

    @property({ type: Boolean })
    premium: boolean = false;

    @property({ type: Boolean, reflect: true })
    flipped: boolean = false;

    @query("carousel-single")
    carousel!: CarouselSingle;

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
        let jiggling_file = this.kind === 'jig' ? "jig-jiggling.svg"
            : this.kind === "resource" ? "resource-jiggling.png"
            : this.kind === "playlist" ? "playlist-jiggling.png"
            : "";

        return html`
            <div class="wrapper">
                <div class="premium">
                    ${this.premium
                        ? html`<img-ui
                                path="icons/pro-icon.svg"
                            ></img-ui>`
                        : nothing
                    }
                </div>
                <img-ui
                    class="jiggling"
                    path="search/cards/${jiggling_file}"
                ></img-ui>
                <slot name="front"></slot>
                <div class="back">
                    <div class="scrollable-content">
                        ${ this.kind === "jig" ? html`
                            <div class="thumbnails">
                                <fa-button
                                    @click=${() => this.carousel.back()}
                                    icon="fa-regular fa-angle-left"
                                ></fa-button>
                                <carousel-single>
                                    <slot name="thumbnails"></slot>
                                </carousel-single>
                                <fa-button
                                    @click=${() => this.carousel.forward()}
                                    icon="fa-regular fa-angle-right"
                                ></fa-button>
                            </div>
                        ` : nothing }
                        <h3 class="name" dir="auto">${this.name}</h3>
                        <div class="published-at">
                            <img-ui path="entry/home/search-results/clock.svg"></img-ui>
                            ${this.publishedAt}
                        </div>
                        <div class="collapsibles">
                            <slot name="categories"></slot>
                            <home-search-result-details>
                                <h4>${STR_DESCRIPTION}</h4>
                                <p class="description" dir="auto">${this.description}</p>
                            </home-search-result-details>
                            ${
                                this.showAdditionalResources ? html`
                                    <home-search-result-details>
                                        <h4>${STR_ADDITIONAL_RESOURCES}</h4>
                                        <div class="additional-resources-items">
                                            <slot name="additional-resources"></slot>
                                        </div>
                                    </home-search-result-details>
                                ` : nothing
                            }
                            <div class="author-section">
                                <a class="left-side" href="${this.authorLink}">
                                    ${
                                        this.byJiTeam ? html`
                                            <img-ui
                                                path="entry/home/search-results/ji-logo-white.svg"
                                            ></img-ui>
                                            <span class="by-ji-team"
                                                >${STR_JI_TEAM}
                                            </span>
                                        ` : nothing
                                    }
                                    ${this.authorName}
                                </a>
                                <!-- <a>
                                    ${STR_SEE_ALL}
                                    <fa-icon
                                        icon="fa-light fa-chevron-right"
                                    ></fa-icon>
                                </a> -->
                            </div>
                        </div>
                    </div>
                    <div class="result-actions">
                        <div class="extra-actions">
                            <slot name="actions"></slot>
                        </div>
                        <div class="primary-action">
                            <slot name="play-button"></slot>
                        </div>
                    </div>
                </div>
            </div>
        `;
    }
}
