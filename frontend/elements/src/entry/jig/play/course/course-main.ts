import { css, customElement, html, LitElement, property } from "lit-element";

@customElement("jig-play-course-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 4fr 6fr;
                    column-gap: 48px;
                    background-color: var(--light-blue-1);
                    min-height: 100vh;
                    padding: 30px 60px;
                }
                ::slotted([slot=thumbnail]) {
                    width: 100%;
                }
                .name {
                    font-size: 32px;
                    font-weight: 900;
                    margin: 0;
                    color: var(--dark-blue-4);
                }
                .count-lang-author {
                    display: grid;
                    justify-content: start;
                    grid-template-columns: repeat(3, auto);
                    column-gap: 16px;
                    color: var(--dark-gray-6);
                }
                .items-count {
                    font-size: 18px;
                    font-weight: 600;
                }
                .description {
                    color: var(--dark-gray-6);
                    margin: 0;
                }
                .additional-resources h3 {
                    background-color: var(--light-blue-1);
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

    render() {
        return html`
            <div class="course-info-side">
                <slot name="thumbnail"></slot>
                <h2 class="name">${this.name}</h2>
                <div class="count-lang-author">
                    <span class="items-count">
                        <img-ui path=""></img-ui>
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