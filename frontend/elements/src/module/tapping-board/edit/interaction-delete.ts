import { LitElement, html, css, customElement } from "lit-element";

const STR_DELETE = "Delete";

@customElement("interaction-delete-action")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    justify-self: start;
                }
                .content {
                    display: flex;
                    align-items: center;
                    column-gap: 3px;
                }
            `,
        ];
    }

    render() {
        return html`
            <button-rect kind="text" color="blue">
                <div class="content">
                    <img-ui
                        path="module/_common/edit/widgets/sidebar/icons/delete.svg"
                    ></img-ui>
                    ${STR_DELETE}
                </div>
            </button-rect>
        `;
    }
}

