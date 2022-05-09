import { LitElement, html, css, customElement, property } from "lit-element";
import { IconKind, IconSize } from "./icon";
import "./icon";

export type LabelColor = "blue";

@customElement("button-icon-label")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host, a {
                    display: inline-flex;
                    align-items: center;
                    gap: 8px;
                    cursor: pointer;
                }

                a {
                    text-decoration: none;
                }

                :host([labelColor="blue"]) .label {
                    color: var(--main-blue);
                }

                .label {
                    user-select: none;
                }

                :host([size="small"]) .label {
                    font-size: 16px;
                }
                :host([size="medium"]) .label {
                    font-size: 18px;
                }
            `,
        ];
    }

    @property()
    icon: IconKind = "circle-check";

    @property()
    iconPath?: string;

    @property()
    iconHoverPath?: string;

    @property()
    iconActivePath?: string;

    @property()
    size: IconSize | undefined;

    @property({ type: Boolean, reflect: true })
    hover: boolean = false;

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    @property({ type: String })
    href?: string;

    @property()
    label: string = "";

    @property({ reflect: true })
    labelColor: LabelColor = "blue";

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
        this.hover = true;
    }

    onMouseLeave() {
        this.hover = false;
    }

    render() {
        const { icon, active, hover, label, size, iconPath, iconHoverPath, iconActivePath } = this;

        const inner = html`
            <button-icon
                .icon=${icon}
                .iconPath=${iconPath}
                .iconHoverPath=${iconHoverPath}
                .iconActivePath=${iconActivePath}
                .active=${active}
                .hover=${hover}
                .size=${size}
                disableHover
            ></button-icon>
            <div class="label">${label}</div>
        `;

        return this.href ? html`<a href=${this.href}>${inner}</a>` : inner;
    }
}
