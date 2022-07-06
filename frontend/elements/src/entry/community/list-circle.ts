import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("community-list-circle")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                p {
                    margin: 0;
                }
                :host {
                    min-height: 136px;
                    padding: 0 24px;
                    display: grid;
                    align-items: center;
                    justify-content: space-between;
                    column-gap: 24px;
                    border-radius: 16px;
                    box-shadow: 0 0 6px 0 rgba(0, 0, 0, 0.08);
                    border: solid 1px var(--light-orange-3);
                    background-color: #ffffff;
                    cursor: pointer;
                }
                ::slotted([slot=img]) {
                    height: 104px;
                    width: 104px;
                    border-radius: 50%;
                    overflow: hidden;
                    border: solid 2px #faef9c;
                }
                .name {
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                }
                .member-count {
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                }
                .member-count fa-icon {
                    color: var(--main-blue);
                }
                .description {
                    font-size: 14px;
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
            <p class="member-count">
                <fa-icon icon="fa-thin fa-people-group"></fa-icon>
                ${this.memberCount}
            </p>
            <p class="description">${this.description}</p>
            <slot name="status"></slot>
        `;
    }
}
