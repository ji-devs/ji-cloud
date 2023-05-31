import { LitElement, html, css, customElement, property } from "lit-element";

export type Kind = "jig" | "resource" | "playlist";

const STR_JIGS = "JIGs";
const STR_RESOURCES = "Resource Library";
const STR_PLAYLISTS = "Playlists";

const IMAGE_LOOKUP: {
    [key in Kind]: string;
} = {
    ["jig"]: "jig-section.png",
    ["resource"]: "resources.webp",
    ["playlist"]: "playlist-section.svg",
};

@customElement("home-search-results-section")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    row-gap: 48px;
                    max-width: 1800px;
                    margin: 0 auto;
                    padding: 5px 40px;
                }
                @media (min-width: 1024px) {
                    :host {
                        padding: 5px 60px;
                    }
                }
                :host([kind=resource]) {
                    background-color: var(--green-2);
                }
                .top-line {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }
                .left-side {
                    display: flex;
                    position: relative;
                }
                .left-side img-ui {
                    position: absolute;
                    right: 100%;
                    height: 45px;
                }
                h2 {
                    margin: 0;
                    font-size: 24px;
                    font-weight: 800;
                    color: var(--dark-blue-4);
                }
                .results-count {
                    font-size: .6em;
                    font-weight: 500;
                }
                .rated {
                    margin: 0 auto;
                    width: 100%;
                    max-width: 512px;
                }
                .results {
                    /* display: flex;
                    flex-wrap: wrap; */
                    display: grid;
                    grid-template-columns: repeat(auto-fill, 280px);
                    /* justify-content: space-between; */
                    justify-content: space-evenly;
                    row-gap: 60px;
                    column-gap: 30px;
                }
                :host([dense]) .results {
                    grid-template-columns: repeat(auto-fill, 216px);
                }
                .results ::slotted(*) {
                    margin: 0 auto;
                }
                .load-more {
                    display: grid;
                    place-content: center;
                }
                .load-more ::slotted(*) {
                    margin-bottom: 40px;
                }

                /* mobile */
                /* @media (max-width: 1000px) {
                    :host {
                        padding: 5px;
                    }
                    .top-line {
                        justify-content: center;
                    }
                    .left-side img-ui {
                        position: static;
                    }
                    h2 {
                        font-size: 40px;
                    }
                    .results {
                        justify-content: center;
                    }
                } */
            `,
        ];
    }

    @property({ reflect: true })
    kind: Kind = "jig";

    @property({ type: Number })
    resultsCount: number = 0;

    @property({ type: Boolean, reflect: true })
    dense: boolean = false;

    render() {
        return html`
            <div class="top-line">
                <div class="left-side">
                    <img-ui
                        path="entry/home/search-results/${IMAGE_LOOKUP[this.kind]}"
                    ></img-ui>
                    <h2>
                        ${
                            this.kind === "jig" ? STR_JIGS
                                : this.kind === "resource" ? STR_RESOURCES
                                : STR_PLAYLISTS
                        }
                        <span class="results-count">
                            (${this.resultsCount})
                        </span
                        >
                    </h2>
                </div>
                <slot name="sort"></slot>
            </div>
            <div class="rated">
                <slot name="rated"></slot>
            </div>
            <div class="results">
                <slot name="results"></slot>
            </div>
            <div class="load-more">
                <slot name="load-more"></slot>
            </div>
        `;
    }
}
