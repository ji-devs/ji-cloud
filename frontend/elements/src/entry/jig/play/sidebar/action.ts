import { LitElement, html, css, customElement, property } from "lit-element";

export type Kind = "like" | "share" | "info";

@customElement("jig-play-sidebar-action")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    height: 48px;
                    width: 48px;
                    display: grid;
                    place-content: center;
                    border: 0;
                    background-color: transparent;
                    border-radius: 50%;
                    cursor: pointer;
                    background-color: #ffffff;
                }
                button:hover {
                    background-color: #f7f7f7;
                }
            `,
        ];
    }

    @property()
    kind: Kind = "like";

    @property({ type: Boolean })
    active: boolean = false;

    render() {
        return html`
            <button>
                <img-ui
                    path="entry/jig/play/sidebar/action-${this.kind}${this
                        .active
                        ? "-active"
                        : ""}.svg"
                ></img-ui>
            </button>
        `;
    }
}
