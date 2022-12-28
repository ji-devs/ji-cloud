import { LitElement, html, css, customElement, property } from "lit-element";
import { listItemStyles } from "./list-item-styles";

@customElement("community-list-circle")
export class _ extends LitElement {
    static get styles() {
        return [
            listItemStyles,
            css`
                p {
                    margin: 0;
                }
                :host {
                    min-height: 106px;
                    border-radius: 12px;
                    box-shadow: 0 0 6px 0 rgba(0, 0, 0, 0.08);
                    border: solid 1px var(--light-orange-3);
                    background-color: #ffffff;
                    cursor: pointer;
                }
                ::slotted([slot=img]) {
                    height: 80px;
                    width: 80px;
                    border-radius: 50%;
                    overflow: hidden;
                    border: solid 2px #faef9c;
                }
                .name {
                    font-size: 14px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                }
                .member-count {
                    font-size: 14px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                }
                .member-count fa-icon {
                    color: var(--main-blue);
                }
                .description {
                    font-size: 12px;
                    color: var(--dark-gray-6);
                }
            `,
        ];
    }

    @property()
    name: string = "";

    @property({ type: Number })
    memberCount: number = 0;

    @property()
    description: string = "";

    render() {
        return html`
            <slot name="img"></slot>
            <p class="name">${this.name}</p>
            <p class="desktop-only member-count">
                <fa-icon icon="fa-thin fa-people-group"></fa-icon>
                ${this.memberCount}
            </p>
            <p class="desktop-only description">${this.description}</p>
            <slot class="desktop-only status" name="status"></slot>
        `;
    }
}
