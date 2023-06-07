import { css, customElement, html, LitElement, property } from "lit-element";

@customElement("jig-play-playlist-item")
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
                    row-gap: 4px;
                    color: var(--dark-gray-6);
                    grid-template-rows: auto auto auto;
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

                ::slotted([slot="read-more"]) {
                    color: #5590fc;
                    font-size: 13px;
                }

                ::slotted([slot="read-more"]:hover) {
                    color: #fc7f55;
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

    @property()
    hideDescription: boolean = false;

    render() {
        const renderDescription = () => {
            if (this.hideDescription === true && this.description.length >= 100) {
                return html`
                    <div class="description" dir="auto">
                    ${this.description.substring(0, 100)}
                    <button-empty slot="read-more"></button-empty>
                </div>
                `;
            } else {
                return html`
                    <div class="description" dir="auto">${this.description}</div>
                `;
            }
        };


        return html`
            <div class="index">${
                this.done ? html`<fa-icon icon="fa-solid fa-check"></fa-icon>` : this.index
            }</div>
            <slot name="thumbnail"></slot>
            <div class="column-3">
                <div class="name" dir="auto">${this.name}</div>
                ${renderDescription()}
            </div>
            <slot name="popup-info"></slot>
            <slot name="play-button"></slot>
        `;
    }
}
