import {
    LitElement,
    html,
    css,
    customElement,
    property,
    internalProperty,
} from "lit-element";

@customElement("jig-play-play-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    width: 160px;
                    height: 160px;
                    box-shadow: 0 3px 80px 0 rgba(0, 0, 0, 0.4);
                    border-radius: 50%;
                    background-color: #f84f57;
                    border: none;
                    cursor: pointer;
                }
                img-ui {
                    height: 80px;
                }
            `,
        ];
    }

    @internalProperty()
    active: boolean = false;

    onMouseEnter() {
        this.active = true;
    }

    onMouseLeave() {
        this.active = false;
    }

    render() {
        return html`
            <button
                @mouseenter="${this.onMouseEnter}"
                @mouseleave="${this.onMouseLeave}"
            >
                <img-ui
                    path="entry/jig/play/play${this.active
                        ? "-active"
                        : ""}.svg"
                ></img-ui>
            </button>
        `;
    }
}
