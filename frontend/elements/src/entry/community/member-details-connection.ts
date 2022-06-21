import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("community-member-details-connection")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    height: 112px;
                    display: grid;
                    grid-template-columns: auto 1fr auto;
                    align-items: center;
                    column-gap: 24px;
                    cursor: pointer;
                }
                :host(:not(:last-of-type)) {
                    border-bottom: solid 1px #ffe2bf;
                }
                ::slotted([slot=profile-image]) {
                    height: 80px;
                    width: 80px;
                }
                p {
                    margin: 0;
                    font-size: 16px;
                    font-weight: 500;
                }
            `,
        ];
    }

    @property()
    name: string = "";

    render() {
        return html`
            <slot name="profile-image"></slot>
            <p>${this.name}</p>
            <slot name="follow"></slot>
        `;
    }
}
