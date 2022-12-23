import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing, Part } from "lit-html";
import { classMap } from "lit-html/directives/class-map";
import "@elements/core/images/ui";

export type IconSize = "x-small" | "small" | "medium";

export type IconKind =
    | "circle-x-blue"
    | "circle-+-blue"
    | "circle-check"
    | "circle-kebab-grey"
    | "circle-kebab-blue"
    | "circle-pencil"
    | "audio"
    | "white-circle-blue-arrow"
    | "audio-stop"
    | "gears"
    | "x";

/* Add the icons to the supported states
  If it's not supported then it will just fall back to regular
*/
const hoverSet: Set<IconKind> = new Set();
hoverSet.add("audio");

const activeSet: Set<IconKind> = new Set();
const wrapperSet: Set<IconKind> = new Set();

wrapperSet.add("x");

@customElement("button-icon")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    cursor: pointer;
                    width: var(--button-width, 30px);
                    height: var(--button-height, 30px);
                }
                :host([size="x-small"]) {
                    width: 16px;
                    height: 16px;
                }
                :host([size="small"]) {
                    width: 24px;
                    height: 24px;
                }
                :host([size="medium"]) {
                    width: 32px;
                    height: 32px;
                }

                /* Used to create a wrapper that will still register
          pointer events even if the contents are smaller than the button size
        */
                .wrapper {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    width: inherit;
                    height: inherit;
                }

                img-ui.visible {
                    display: inherit;
                }

                img-ui {
                    display: none;
                    width: inherit;
                    height: inherit;
                    object-fit: inherit;
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

    @property({ reflect: true })
    size: IconSize | undefined;

    @property({ type: Boolean, reflect: true })
    hover: boolean = false;

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    @property({ type: Boolean })
    disableHover: boolean = false;

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
        if (!this.disableHover) {
            this.hover = true;
        }
    }

    onMouseLeave() {
        if (!this.disableHover) {
            this.hover = false;
        }
    }

    renderKnownIcon() {
        const { icon, hover, active } = this;
        const filename =
            icon === "circle-check"
                ? "circle-check-green"
                : icon === "circle-kebab-grey"
                ? "circle-kebab-grey"
                : icon === "circle-kebab-blue"
                ? "circle-kebab-blue"
                : icon === "circle-pencil"
                ? "circle-pencil-blue"
                : icon === "gears"
                ? "gears-plus-blue"
                : icon;

        const basePath = `core/buttons/icon`;

        const isActive = activeSet.has(icon) && active;
        const isHover = hoverSet.has(icon) && hover;

        const regularClasses = classMap({
            visible: !isActive && !isHover,
        });
        const hoverClasses = classMap({
            visible: isHover && !isActive,
        });
        const activeClasses = classMap({
            visible: isActive,
        });

        const regularImage = html`<img-ui
            class=${regularClasses}
            path="${basePath}/${filename}.svg"
        ></img-ui>`;
        const hoverImage = hoverSet.has(icon)
            ? html`<img-ui
                class=${hoverClasses}
                path="${basePath}/${filename}-hover.svg"
            ></img-ui>`
            : nothing;
        const activeImage = activeSet.has(icon)
            ? html`<img-ui
                class=${activeClasses}
                path="${basePath}/${filename}-active.svg"
            ></img-ui>`
            : nothing;

        const images = html` ${regularImage} ${hoverImage} ${activeImage} `;

        /* to try and minimize missing image flashes, load them all
        TODO - sprite sheets
        */
        return wrapperSet.has(icon)
            ? html`<div class="wrapper">${images}</div>`
            : images;
    }

    renderIconFromPath() {
        const { iconPath, iconHoverPath, iconActivePath, hover, active } = this;

        const regularClasses = classMap({
            visible: !active && !hover,
        });
        const hoverClasses = classMap({
            visible: hover && !active,
        });
        const activeClasses = classMap({
            visible: active,
        });

        const regularImage = html`<img-ui
            class=${regularClasses}
            path=${iconPath}
        ></img-ui>`;
        const hoverImage = iconHoverPath
            ? html`<img-ui
                class=${hoverClasses}
                path=${iconHoverPath}
            ></img-ui>`
            : nothing;
        const activeImage = iconActivePath
            ? html`<img-ui
                class=${activeClasses}
                path=${iconActivePath}
            ></img-ui>`
            : nothing;

        return html` ${regularImage} ${hoverImage} ${activeImage} `;
    }

    render() {
        if (this.iconPath) {
            return this.renderIconFromPath();
        } else {
            return this.renderKnownIcon();
        }
    }
}
