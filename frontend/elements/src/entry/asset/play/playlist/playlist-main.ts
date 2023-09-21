import { css, customElement, html, LitElement, property } from "lit-element";

@customElement("jig-play-playlist-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    align-content: start;
                    column-gap: 48px;
                    background-color: var(--light-blue-1);
                    min-height: 100dvh;
                    box-sizing: border-box;
                }
                @media (min-width: 1024px) {
                    :host {
                        padding: 24px 48px;
                        grid-template-columns: 4fr 6fr;
                    }
                }
                .playlist-info-side {
                    display: grid;
                    align-content: start;
                    row-gap: 12px;
                    justify-content: center;
                    text-align: center;
                    padding: 8px;
                }
                @media (min-width: 1024px) {
                    .playlist-info-side {
                        justify-content: auto;
                        text-align: left;
                        padding: 0;
                    }
                }
                .top-bar .logo {
                    height: 14px;
                }
                @media (min-width: 1024px) {
                    .top-bar .logo {
                        height: 28px;
                    }
                }
                ::slotted([slot=thumbnail]) {
                    max-height: 50vh;
                    aspect-ratio: 16 / 9;
                }
                .name {
                    font-size: 28px;
                    font-weight: 900;
                    margin: 0;
                    color: var(--dark-blue-4);
                }
                .count-lang-author {
                    display: grid;
                    align-items: center;
                    column-gap: 12px;
                    row-gap: 6px;
                    color: var(--dark-gray-6);
                    font-size: 14px;
                    justify-content: center;
                }
                @media (min-width: 1024px) {
                    .count-lang-author {
                        justify-content: start;
                        grid-template-columns: repeat(4, auto);
                    }
                }
                .count-lang-author img-ui {
                    display: none;
                }
                @media (min-width: 1024px) {
                    .count-lang-author img-ui {
                        display: inline-block;
                        height: 20px;
                    }
                }
                .items-count {
                    font-size: 16px;
                    font-weight: 600;
                }
                .description {
                    color: var(--dark-gray-6);
                    margin: 0;
                    overflow-wrap: break-word;
                    white-space: pre-wrap;
                }
                :host(:not([hasAdditionalResources])) .additional-resources {
                    display: none;
                }
                .additional-resources h3 {
                    background-color: var(--light-blue-1);
                }
                ::slotted(a[slot=additional-resources]) {
                    color: var(--main-blue);
                    font-weight: 500;
                    text-decoration: none;
                    font-size: 12px;
                    display: flex;
                    column-gap: 5px;
                }
                @media (min-width: 1024px) {
                    .playlist-items {
                        border-radius: 12px;
                        overflow: hidden;
                    }
                }
                .items-header {
                    background-color: var(--light-blue-3);
                    padding: 20px 16px;
                    display: grid;
                    grid-template-columns: auto auto;
                    align-items: center;
                    justify-content: space-between;
                }
                .items-header ::slotted([slot=play]) {
                    height: 50px;
                    width: 50px;
                    border-radius: 50%;
                    background-color: #fff;
                    color: var(--dark-blue-8);
                    display: grid;
                    place-content: center;
                    font-size: 50px;
                }
                .items-header ::slotted([slot=share]) {
                    color: var(--main-blue);
                    font-size: 18px;
                    font-weight: 500;
                    display: flex;
                    column-gap: 8px;
                }
            `,
        ];
    }

    @property()
    name: string = "";

    @property()
    description: string = "";

    @property()
    language: string = "";

    @property()
    author: string = "";

    @property({ type: Number })
    itemsCount: number = 0;

    @property({ type: Boolean, reflect: true })
    hasAdditionalResources: boolean = false;

    @property()
   itemType: string = "";

    render() {
        return html`
            <div class="playlist-info-side">
                <a href="/" target="top">
                    <img-ui class="logo" path="jig/play/logo.svg"></img-ui>
                </a>
                <slot name="thumbnail"></slot>
                <h2 class="name" dir="auto">${this.name}</h2>
                <div class="count-lang-author">
                    <img-ui path="entry/home/search-results/jig-section.png"></img-ui>
                    <span class="items-count">
                        ${this.itemsCount} ${this.itemType}
                    </span>
                    <span class="language">
                        <fa-icon icon="fa-light fa-globe"></fa-icon>
                        ${this.language}
                    </span>
                    <span class="author">
                        Created by: ${this.author}
                    </span>
                </div>
                <p class="description" dir="auto">${this.description}</p>
                <div class="additional-resources">
                    <h3>Teacher resources</h3>
                    <slot name="additional-resources"></slot>
                </div>
            </div>
            <div class="playlist-items">
                <div class="items-header">
                    <slot name="play"></slot>
                    <slot name="share"></slot>
                </div>
                <slot name="items"></slot>
            </div>
        `;
    }
}
