import { css, customElement, html, LitElement, property } from "lit-element";

@customElement("jig-play-course-item")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: repeat(2, auto) 1fr auto;
                    column-gap: 16px;
                    padding: 16px;
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
                    width: 30px;
                    height: 30px;
                    border-radius: 50%;
                    border: solid 1px var(--dark-blue-1);
                }
                :host([done]) .index {
                    background-color: var(--dark-green-1);
                    color: #fff;
                    border: none;
                }
                ::slotted([slot=thumbnail]) {
                    width: 150px;
                    border-radius: 10px;
                    aspect-ratio: 16 / 9;
                }
                .column-3 {
                    display: grid;
                    row-gap: 8px;
                    color: var(--dark-gray-6);
                }
                .name {
                    font-size: 16px;
                    font-weight: 600;
                }
                .description {
                    display: none;
                }
                @media (min-width: 1024px) {
                    .description {
                        display: block;
                        overflow-wrap: break-word;
                        white-space: pre-wrap;
                        font-size: 12px;
                    }
                }
                ::slotted([slot=play-button]) {
                    height: 30px;
                    width: 30px;
                    font-size: 17px;
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

    @property({ type: Boolean, reflect: true })
    done: boolean = false;

    render() {
        return html`
            <div class="index">${
                this.done ? html`<fa-icon icon="fa-solid fa-check"></fa-icon>` : this.index
            }</div>
            <slot name="thumbnail"></slot>
            <div class="column-3">
                <div class="name" dir="auto">${this.name}</div>
                <div class="description" dir="auto">${this.description}</div>
            </div>
            <slot name="play-button"></slot>
        `;
    }
}
