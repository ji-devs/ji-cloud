import {
    LitElement,
    html,
    css,
    customElement,
    property,
    internalProperty,
} from "lit-element";
import "@elements/core/images/ui";

const STR_HERE_TO_HELP = "Here to help";

@customElement("home-search-section-help")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                    grid-template-columns: auto auto;
                    align-items: flex-start;
                    cursor: pointer;
                }
                .tooltip {
                    background-color: var(--orange-1);
                    border-radius: 16px;
                    border-bottom-right-radius: 0;
                    padding: 6px 8px;
                    color: #ffffff;
                    font-size: 14px;
                    font-weight: bold;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    grid-column: 1;
                    grid-row: 1;
                    position: relative;
                    transition: transform 0.2s;
                }
                :host([active]) .tooltip {
                    background-color: var(--dark-red-1);
                    transform: rotate(10deg);
                }
                .tooltip::after {
                    content: "";
                    display: inline-block;
                    border: 6px solid transparent;
                    border-top-color: var(--orange-1);
                    border-right-color: var(--orange-1);
                    position: absolute;
                    right: 0;
                    top: 32px;
                }
                :host([active]) .tooltip::after {
                    border-top-color: var(--dark-red-1);
                    border-right-color: var(--dark-red-1);
                }
                img-ui {
                    grid-column: 1 / -1;
                    grid-row: 1;
                    margin-left: 60px;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    connectedCallback() {
        super.connectedCallback();

        this.addEventListener("mouseenter", this.onMouseEnter);
        this.addEventListener("mouseleave", this.onMouseLeave);
    }

    disconnectedCallback() {
        super.disconnectedCallback();

        this.removeEventListener("mouseenter", this.onMouseEnter);
        this.removeEventListener("mouseleave", this.onMouseLeave);
    }

    onMouseEnter() {
        this.active = true;
    }

    onMouseLeave() {
        this.active = false;
    }

    render() {
        return html`
            <span class="tooltip">${STR_HERE_TO_HELP}</span>
            <img-ui
                path="entry/home/search-section/here-to-help${this.active
                    ? "-wink"
                    : ""}.png"
            ></img-ui>
        `;
    }
}
