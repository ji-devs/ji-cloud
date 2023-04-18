import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

const STR_PLAYED = "Plays";
const STR_VIEWED = "Views";
const STR_LIKED = "Likes";

type Kind = "jig" | "resource" | "course";

@customElement("asset-card")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    border-radius: 20px;
                    box-shadow: 2px 3px 10px 0 rgba(215, 215, 215, 0.5);
                    overflow: hidden;
                    background-color: white;
                    width: 280px;
                    height: 246px;
                    grid-template-rows: auto 0px 1fr;
                }
                :host([dense]) {
                    width: 216px;
                }
                :host([showBottomIndicator]) {
                    height: 278px;
                    grid-template-rows: auto 0px 1fr 32px;
                }
                ::slotted([slot=menu]) {
                    grid-row: 1;
                    grid-column: 1;
                    justify-self: end;
                    align-self: start;
                    margin: 8px;
                    border-radius: 50%;
                    z-index: 1;
                }
                ::slotted([slot=image]) {
                    grid-row: 1;
                    grid-column: 1;
                    aspect-ratio: 16 / 9;
                }
                .middle {
                    height: 30px;
                    translate: 0 -50%;
                    display: grid;
                    grid-template-columns: 30px auto 30px;
                    column-gap: 8px;
                    justify-content: center;
                }
                :host([kind=jig]) .middle {
                    /* jig doesn't have a middle indicator */
                    grid-template-columns: 30px;
                }
                .middle ::slotted([slot=like]) {
                    border-radius: 50%;
                    border: solid 1px #5590fc;
                    background-color: #fff;
                    display: inline-grid;
                    place-content: center;
                    font-size: 18px;
                }
                .middle ::slotted([slot=like][icon^=fa-regular]) {
                    color: #5590fc;
                }
                .middle ::slotted([slot=like][icon^=fa-solid]) {
                    color: #fa4048;
                }
                .middle ::slotted([slot=middle-indicator]) {
                    grid-column: 2;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: #fee13c;
                    border-radius: 15px;
                    font-size: 12px;
                    font-weight: bold;
                    text-align: center;
                    color: var(--dark-gray-5);
                    padding: 0 8px;
                    line-height: 30px;
                }
                .white-section {
                    padding: 6px;
                    line-height: 1em;
                    display: grid;
                    align-content: space-between;
                }
                :host([hasMiddleIndicator]) .white-section {
                    /* add top padding when middle-indicator is present */
                    padding-top: 16px;
                }
                .name {
                    font-size: 16px;
                    font-weight: 600;
                    color: #555;
                    text-align: center;
                    margin: 0;

                    /* max 2 lines */
                    display: -webkit-box;
                    -webkit-line-clamp: 2;
                    -webkit-box-orient: vertical;
                    overflow: hidden;
                    text-overflow: ellipsis;
                }
                .played-liked {
                    font-size: 13px;
                    color: var(--dark-gray-5);
                    display: flex;
                    justify-content: center;
                    column-gap: 8px;
                }
                .played-liked .count {
                    font-weight: 800;
                }
                .played-liked .played-liked-divider {
                    width: 2px;
                    height: 16px;
                    background-color: var(--dark-gray-5);
                }
                .ages-language {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                }
                .language {
                    font-size: 13px;
                    font-weight: 500;
                    color: var(--dark-gray-3);
                    grid-column: 2;
                }
                .bottom-indicator {
                    display: none;
                }
                :host([showBottomIndicator]) .bottom-indicator {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    column-gap: 18px;
                    font-size: 13px;
                    font-weight: 500;
                    color: var(--dark-gray-5);
                }
                :host([kind=jig]) .bottom-indicator {
                    background-color: #fff6d9;
                }
                :host([kind=resource]) .bottom-indicator {
                    background-color: #e3f5fd;
                }
                :host([kind=course]) .bottom-indicator {
                    background-color: #e9fae5;
                }
            `,
        ];
    }

    @property({ reflect: true })
    kind: Kind = "jig";

    @property({ type: Boolean, reflect: true })
    dense: boolean = false;

    @property()
    name: string = "";

    @property({ type: Number })
    playedCount: number = 0;

    @property({ type: Number })
    likedCount: number = 0;

    @property()
    language: string = "";

    @property({ type: Boolean, reflect: true })
    showBottomIndicator: boolean = true;

    @property({ type: Boolean, reflect: true })
    private hasMiddleIndicator: boolean = false;

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
            <slot
                name="menu" 
                @click=${(e: Event) => {
                    e.preventDefault();
                    e.stopPropagation();
                }}
                @pointerdown=${(e: Event) => {
                    e.preventDefault();
                    e.stopPropagation();
                }}
            ></slot>
            <slot name="image"></slot>
            <div class="middle">
                <slot name="like"></slot>
                <slot
                    @slotchange=${(e: any) => this.hasMiddleIndicator = e.target.assignedNodes().length > 0}
                    name="middle-indicator"
                ></slot>
            </div>
            <div class="white-section">
                <h3 class="name">${this.name}</h3>
                <div class="played-liked">
                    ${this.renderCount(playedLabel, this.playedCount)}
                    ${this.renderCountDivider()}
                    ${this.renderCount(STR_LIKED, this.likedCount)}
                </div>
                <div class="ages-language">
                    <slot name="ages"></slot>
                    <div class="language">
                        <fa-icon icon="fa-light fa-globe"></fa-icon>
                        <span class="language-code">${this.language}</span>
                    </div>
                </div>
            </div>
            <div class="bottom-indicator">
                <slot name="bottom-indicator"></slot>
            </div>
        `;
    }
}
