import { css, customElement, html, LitElement, property } from "lit-element";

@customElement("jig-play-course-item")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: repeat(2, auto) 1fr auto;
                    column-gap: 20px;
                    padding: 20px;
                    cursor: pointer;
                    align-items: center;
                    background-color: #fff;
                }
                .index {
                    color: var(--dark-blue-1);
                    font-size: 16px;
                    font-weight: bold;
                    display: inline-grid;
                    place-content: center;
                    width: 10px;
                }
                @media (min-width: 1920px) {
                    .index {
                        width: 40px;
                        height: 40px;
                        border-radius: 50%;
                        border: solid 1px var(--dark-blue-1);
                    }
                }
                ::slotted([slot=thumbnail]) {
                    width: 190px;
                    border-radius: 10px;
                    aspect-ratio: 16 / 9;
                }
                .column-3 {
                    display: grid;
                    row-gap: 10px;
                    color: var(--dark-gray-6);
                }
                .name {
                    font-size: 18px;
                    font-weight: 600;
                }
                .description {
                    display: none;
                }
                @media (min-width: 1920px) {
                    .description {
                        display: block;
                        overflow-wrap: break-word;
                        white-space: pre-wrap;
                    }
                }
                ::slotted([slot=play-button]) {
                    height: 40px;
                    width: 40px;
                    background-color: var(--main-blue);
                    color: white;
                    border-radius: 50%;
                    display: inline-grid;
                    place-content: center;
                }
            `,
        ];
    }

    @property({ type: Number })
    index: number = 0;

    @property()
    name: string = "";

    @property()
    description: string = "";

    render() {
        return html`
            <div class="index">${this.index}</div>
            <slot name="thumbnail"></slot>
            <div class="column-3">
                <div class="name">${this.name}</div>
                <div class="description">${this.description}</div>
            </div>
            <slot name="play-button"></slot>
        `;
    }
}
