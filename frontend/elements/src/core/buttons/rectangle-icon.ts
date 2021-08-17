import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from 'lit-html';
import { ifDefined } from 'lit-html/directives/if-defined';
import { Color, Size } from './rectangle';

export type IconAfter = "arrow" | "done";
export type IconBefore = "magnifier" | "share" | "create" | "play" | "plus";

@customElement("button-rect-icon")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .content {
                    display: inline-flex;
                    column-gap: 8px;
                    align-items: center;
                    justify-content: center;
                }
            `
        ];
    }

    @property({ reflect: true })
    size: Size = "medium";

    @property({ reflect: true })
    color: Color = "red";

    @property({ type: Boolean })
    bold: boolean = false;

    @property({ type: Boolean }) // needed?
    italic: boolean = false;

    @property({ type: Boolean })
    disabled: boolean = false;

    @property({ type: Boolean })
    submit: boolean = false;

    @property()
    href?: string;

    @property()
    target?: string;

    @property()
    iconBefore?: IconBefore;
    @property()
    iconAfter?: IconAfter;

    connectedCallback() {
        super.connectedCallback();
        this.addEventListener("click", this.onClick, true);
    }
    disconnectedCallback() {
        super.disconnectedCallback();
        this.removeEventListener("click", this.onClick);
    }
    onClick(e: MouseEvent) {
        if(this.disabled)
            e.stopPropagation();
    }

    render() {
        const { iconBefore, iconAfter, color, disabled } = this;

        const iconBeforePath = iconBefore === "magnifier" ? "core/buttons/rect/magnifier.svg" 
        : iconBefore === "share" ? `core/buttons/rect/share-${color}.svg`
        : iconBefore === "create" ? `core/buttons/rect/plus-${color}.svg`
        : iconBefore === "play" ? `core/buttons/rect/play-${color}.svg`
        : iconBefore === "plus" ? getPlus(color)
        : nothing;
        const iconAfterPath = iconAfter === "arrow" ? getArrow(disabled) 
            : iconAfter === "done" ? "core/buttons/rect/done-check.svg"
            : "";

        return html`
            <button-rect
                kind="filled"
                size="${this.size}"
                color="${this.color}"
                ?submit="${this.submit}"
                ?disabled="${this.disabled}"
                ?bold="${this.bold}"
                ?italic="${this.italic}"
                href="${ifDefined(this.href)}"
                target="${ifDefined(this.target)}"
            >
                <div class="content">
                    ${iconBefore && html`<img-ui path="${iconBeforePath}"></img-ui>`}
                    <slot></slot>
                    ${iconAfter && html`<img-ui path="${iconAfterPath}"></img-ui>`}
                </div>
            </button-rect>
        `;
    }
}
function getPlus(color:Color) {
    return color === "blue" ? "core/inputs/plus-white.svg"
        : nothing;
}
  
function getArrow(disabled: boolean) {
    return disabled ? "core/buttons/rect/arrow-right-white.svg"
        :  "core/buttons/rect/arrow-right-yellow.svg";
}
