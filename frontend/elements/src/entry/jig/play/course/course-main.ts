import { css, customElement, html, LitElement, property } from "lit-element";

@customElement("jig-play-course-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    column-gap: 48px;
                    background-color: var(--light-blue-1);
                    min-height: 100vh;
                }
                @media (min-width: 1920px) {
                    :host {
                        padding: 30px 60px;
                        grid-template-columns: 4fr 6fr;
                    }
                }
                .course-info-side {
                    display: grid;
                    align-content: start;
                    row-gap: 14px;
                    justify-content: center;
                    text-align: center;
                    padding: 10px;
                }
                @media (min-width: 1920px) {
                    .course-info-side {
                        justify-content: auto;
                        text-align: left;
                        padding: 0;
                    }
                }
                ::slotted([slot=thumbnail]) {
                    max-height: 50vh;
                    aspect-ratio: 16 / 9;
                }
                .name {
                    font-size: 32px;
                    font-weight: 900;
                    margin: 0;
                    color: var(--dark-blue-4);
                }
                .count-lang-author {
                    display: grid;
                    align-items: center;
                    grid-template-columns: repeat(4, auto);
                    column-gap: 16px;
                    color: var(--dark-gray-6);
                    justify-content: center;
                }
                @media (min-width: 1920px) {
                    .count-lang-author {
                        justify-content: start;
                    }
                }
                .count-lang-author img-ui {
                    height: 26px;
                }
                .items-count {
                    font-size: 18px;
                    font-weight: 600;
                }
                .description {
                    color: var(--dark-gray-6);
                    margin: 0;
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
                    font-size: 14px;
                    display: flex;
                    column-gap: 6px;
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

    render() {
        return html`
            <div class="course-info-side">
                <slot name="thumbnail"></slot>
                <h2 class="name">${this.name}</h2>
                <div class="count-lang-author">
                    <img-ui path="entry/home/search-results/jig-section.png"></img-ui>
                    <span class="items-count">
                        ${this.itemsCount} Jigs
                    </span>
                    <span class="language">
                        <fa-icon icon="fa-light fa-globe"></fa-icon>
                        ${this.language}
                    </span>
                    <span class="author">
                        Created by: ${this.author}
                    </span>
                </div>
                <p class="description">${this.description}</p>
                <div class="additional-resources">
                    <h3>Additional resources</h3>
                    <slot name="additional-resources"></slot>
                </div>
            </div>
            <div class="course-items">
                <slot name="items"></slot>
            </div>
        `;
    }
}
