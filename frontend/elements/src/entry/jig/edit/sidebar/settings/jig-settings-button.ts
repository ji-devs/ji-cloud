import { LitElement, html, css, customElement, property } from "lit-element";

export type Kind = "theme" | "background" | "feedback"; 

const STR_LABEL: {
    [key in Kind]: string
} = {
    ['theme']: "JIG’s theme",
    ['background']: "Add Background Music",
    ['feedback']: "Feedback Effects",
};

@customElement("jig-settings-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    background-color: transparent;
                    padding: 0;
                    border: 0;
                    cursor: pointer;
                    display: flex;
                    justify-content: space-between;
                    width: 100%;
                    align-items: center;
                }
                button:hover, button:active {
                    color: var(--main-blue);
                }
                .label {
                    display: grid;
                    grid-template-columns: auto auto;
                    column-gap: 12px;
                    align-items: center;
                    font-size: 16px;
                }
                img-ui {
                    display: inline-block;
                    height: 30px;
                    width: 30px;
                    grid-column: 1;
                    grid-row: 1;
                }
                img-ui.active, img-ui.active {
                    display: none
                }
                button:hover img-ui.active, button:active img-ui.active {
                    display: inline-block;
                }
                button:hover img-ui.inactive, button:active img-ui.inactive {
                    display: none;
                }
            `,
        ];
    }

    @property({ reflect: true })
    kind: Kind = "background";

    render() {
        return html`
            <button>
                <div class="label">
                    <img-ui class="inactive" path="entry/jig/settings/${this.kind}.svg"></img-ui>
                    <img-ui class="active" path="entry/jig/settings/${this.kind}-active.svg"></img-ui>
                    <span>${STR_LABEL[this.kind]}</span>
                </div>

                <fa-icon icon="fa-light fa-chevron-right"></fa-icon>
            </button>
        `;
    }
}
