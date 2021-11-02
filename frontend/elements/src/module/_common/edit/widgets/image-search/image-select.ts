import { LitElement, html, css, customElement, property, query } from 'lit-element';
import { nothing } from "lit-html";

export type imageMode = 'image' | 'background';

const STR_MY_RECENT = "My recent";
const STR_SEE_MORE = "See more";
const STR_SEE_LESS = "See less";
const STR_ALL_IMAGES = "All images";
const STR_SEARCH_IN = "Search in";

@customElement("image-select")
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                display: grid;
            }
            :host {
                --image-height: 104px;
            }
            :host([imageMode=image]) {
                --image-width: var(--image-height);
            }
            :host([imageMode=background]) {
                --image-width: 154px;
            }
            @media (min-width: 1920px) {
                :host {
                    --image-height: 118px;
                }
                :host([imageMode=background]) {
                    --image-width: 207px;
                }
            }

            .main {
                grid-row: 1;
                grid-column: 1;
                display: flex;
                flex-direction: column;
                row-gap: 18px;
                column-gap: 24px;
            }
            @media (min-width: 1920px) {
                .main {
                    row-gap: 18px;
                }
            }
            .top-row {
                grid-column: 1 / -1;
                display: flex;
                justify-content: space-between;
            }
            h2 {
                margin: 0;
                font-size: 16px;
                font-weight: normal;
            }
            .controls {
                display: grid;
                row-gap: 6px;
            }
            .search-row {
                display: grid;
                grid-template-columns: auto min-content;
                align-items: center;
            }
            .search-row ::slotted([slot=search-input]) {
                width: 100%;
            }
            .search-row ::slotted([slot=filters]) {
                grid-column: 2;
                margin-left: 24px;
            }
            .bottom-row {
                display: grid;
                grid-template-columns: min-content min-content;
                justify-content: space-between;
                align-items: center;
            }
            .bottom-row ::slotted([slot=upload]) {
                grid-column: 2;
            }
            .source-options {
                display: flex;
                column-gap: 20px;
            }
            h4 {
                white-space: nowrap;
                margin: 0;
                font-size: 16px;
                font-weight: 500;
                color: var(--dark-gray-5);
            }
            ::slotted([slot=source-options]) {
                cursor: pointer;
                display: flex;
                column-gap: 7px;
                align-items: center;
            }
            .images-section {
                padding: 20px 0;
                display: grid;
                grid-template-columns: auto auto;
                justify-content: space-between;
                row-gap: 24px;
            }
            :host([recent]) .images-section {
                border-top: solid 1px #b0ccf2;
            }
            .images-section.recent {
                overflow: hidden;
                max-height: 142px;
            }
            @media (min-width: 1920px) {
                .images-section h4 {
                    max-height: 170px;
                }
            }
            :host([moreShown]) .images-section.recent {
                max-height: revert;
            }
            .images-section h4 {
                font-weight: 600;
                color: #4a4a4a;
                margin: 0;
                font-size: 15px;
            }
            @media (min-width: 1920px) {
                .images-section h4 {
                    font-size: 16px;
                }
            }
            .images-section.recent button-rect .icon {
                display: inline-block;
                transform: rotate(90deg);
                transition: transform .2s;
            }
            :host([moreShown]) .images-section.recent button-rect .icon {
                transform: rotate(-90deg);
            }
            .images-wrapper {
                grid-column: 1 / -1;
                display: grid;
                grid-template-columns: repeat(auto-fit, var(--image-width));
                justify-content: space-between;
                grid-auto-rows: var(--image-height);
                row-gap: 12px;
                column-gap: 10px;
            }
            @media (min-width: 1920px) {
                .images-section {
                    row-gap: 24px;
                    column-gap: 15px;
                }
            }
            ::slotted(img) {
                height: 100%;
                width: 100%;
                object-fit: contain;
                object-position: center;
            }
            ::slotted([slot=images]),
            ::slotted([slot=recent]) {
                display: grid;
                place-content: center;
                cursor: pointer;
                border-radius: 4px;
                transition: transform .2s, box-shadow .2s;
            }
            ::slotted([slot=images]:hover),
            ::slotted([slot=recent]:hover) {
                transform: scale(1.02);
                box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
            }

            .loader-overlay {
                display: none;
                background-color: #0000006e;
                grid-column: 1;
                grid-row: 1;
                place-content: center;
                z-index: 1;
            }
            :host([loading]) .loader-overlay {
                display: grid;
            }
            .loader {
                border: 16px solid #f3f3f3;
                border-top: 16px solid #3498db;
                border-radius: 50%;
                width: 120px;
                height: 120px;
                animation: loader 1s ease-in-out infinite;
            }
            @keyframes loader {
                0% { transform: rotate(0deg); }
                100% { transform: rotate(360deg); }
            }
        `];
    }

    @property({type: String})
    label?: string;

    @property({type: String, reflect: true})
    imageMode: imageMode = "image";

    @property({type: Boolean, reflect: true})
    recent: boolean = false;

    @property({type: Boolean, reflect: true})
    loading: boolean = false;

    @property({type: Boolean, reflect: true})
    private moreShown: boolean = false;

    @query("#main-images-wrapper")
    mainImagesWrapper!: HTMLElement;

    @query("slot[name=images]")
    imagesSlot!: HTMLSlotElement;

    private observer!: IntersectionObserver;

    firstUpdated() {
        this.observer = new IntersectionObserver(this.onIntersection, {
            threshold: 0,
        });
    }

    private onIntersection = (entries: IntersectionObserverEntry[]) => {
        const entry = entries[0];
        if(entry.isIntersecting) {
            this.observer.unobserve(entry.target);

            this.dispatchEvent(new Event("scroll-end"));
        }
    }

    private onNewImage = () => {
        const images = this.imagesSlot.assignedElements();

        if (images.length === 0) return;

        const lastImage = images[images.length - 1];

        this.observer.observe(lastImage);
    }

    render() {
        return html`
            <div class="main">
                <div class="top-row">
                    <h2>${this.label || nothing}</h2>
                    <slot name="hide-overlay"></slot>
                </div>
                <div class="controls">
                    <div class="search-row">
                        <slot name="search-input"></slot>
                        <slot name="filters"></slot>
                    </div>
                    <div class="bottom-row">
                        <div>
                            <div class="source-options">
                                <h4>${STR_SEARCH_IN}</h4>
                                <slot name="source-options"></slot>
                            </div>
                        </div>
                        <slot name="upload"></slot>
                    </div>
                </div>
                <section class="all-images">
                    ${
                        this.recent ? (
                            html`
                                <div class="images-section recent">
                                    <h4>${STR_MY_RECENT}</h4>
                                    <button-rect
                                        kind="text"
                                        color="blue"
                                        @click=${() => this.moreShown = !this.moreShown}
                                    >
                                        ${ this.moreShown ? STR_SEE_LESS : STR_SEE_MORE }
                                        <span class="icon">></span>
                                    </button-rect>
                                    <div class="images-wrapper">
                                        <slot name="recent"></slot>
                                    </div>
                                </div>
                            `
                        ) : nothing
                    }
                    <div class="images-section main">
                        <h4>${STR_ALL_IMAGES}</h4>
                        <div id="main-images-wrapper" class="images-wrapper">
                            <slot name="images" @slotchange=${this.onNewImage}></slot>
                        </div>
                    </div>
                </section>
            </div>
            <div class="loader-overlay">
                <div class="loader"></div>
            </div>
        `;
    }
}
