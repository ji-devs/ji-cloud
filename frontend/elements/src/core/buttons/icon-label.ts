import { LitElement, html, css, customElement, property } from "lit-element";
import { IconKind, IconSize } from "./icon";
import "./icon";

export type LabelColor = "blue" | "dark-blue";

@customElement("button-icon-label")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    --gap-override: 4px;
                }
                :host, a {
                    display: inline-flex;
                    align-items: center;
                    gap: var(--gap-override);
                    cursor: pointer;
                }

                a {
                    text-decoration: none;
                }

                :host([labelColor="blue"]) .label {
                    color: var(--main-blue);
                }

                :host([labelColor="dark-blue"]) .label {
                    color: var(--dark-blue-1);
                }
                :host(:hover[labelColor="dark-blue"]) .label {
                    color: var(--dark-blue-2);
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

    @property({ type: String })
    gapOverride?: string;

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

    updated() {
        if (this.gapOverride) {
            this.style.setProperty("--gap-override", this.gapOverride);
        }
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
