import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import "@elements/core/buttons/icon";
import { collapseStyles } from "../../_common/sidebar-modules/collapse-styles";

@customElement("jig-edit-sidebar-header")
export class _ extends LitElement {
    static get styles() {
        return [
            collapseStyles,
            css`
                :host {
                    padding: 20px;
                    padding-bottom: 0;
                    display: block;

                    transition-timing-function: linear;
                    transition-delay: 0s;
                    transition-duration: var(--collapsing-phase-duration);
                }
                :host([collapsed]) {
                    padding: 10px;
                    transition-delay: var(--fading-phase-duration);
                }
                .close-wrapper {
                    display: flex;
                    justify-content: flex-end;
                    margin-right: -20px;
                    transition-property: margin-top;
                    height: 14px;
                }
                :host([collapsed]) .close-wrapper {
                    margin-right: -10px;
                }
                :host([isModulePage]) .close {
                    opacity: 0;
                    cursor: inherit;
                }
                :host([collapsed]) .close {
                    margin-right: 0px;
                    transition-property: margin-right;
                }
                :host([collapsed]) ::slotted([slot="close"]) {
                    transform: rotate(-180deg);
                }
                ::slotted([slot="close"]) {
                    transition: transform 0.3s;
                }
                .logo-nav-wrapper {
                    margin-top: 16px;
                    height: 40px;
                    display: flex;
                    justify-content: space-between;
                }
                .logo {
                    object-fit: cover;
                    object-position: 0 0;
                    overflow: hidden;
                    height: 40px;
                    width: 115px;
                    transition-property: height, width;
                    transition-duration: var(--collapsing-phase-duration);
                }
                :host([collapsed]) .logo {
                    height: 28px;
                    width: 50px;
                    transition-delay: var(--fading-phase-duration);
                }
                .divider {
                    background-color: #5893f9;
                    height: 12px;
                    width: 1px;
                }
                nav,
                .settings-preview {
                    display: flex;
                    align-items: center;
                    column-gap: 16px;
                    padding: 0;
                    transition-property: column-gap, padding;
                    transition-delay: var(--fading-phase-duration);
                }
                :host([collapsed]) .settings-preview {
                    column-gap: 0px;
                    padding: 0 12px;
                }
                ::slotted([slot="modules"]) {
                    font-size: 24px;
                    color: var(--main-blue);
                }
                .input {
                    margin: 23px 0;
                    width: 100%;
                }
                ::slotted([slot="settings"]) {
                    font-size: 16px;
                    height: 28px;
                    width: 28px;
                    background-color: var(--main-blue);
                    border-radius: 50%;
                    display: inline-grid;
                    place-content: center;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    collapsed: boolean = false;

    @property({ type: Boolean, reflect: true })
    isModulePage: boolean = false;

    render() {
        return html`
            <div class="close-wrapper collapsing-phase">
                <div class="close collapsing-phase">
                    <slot name="close"></slot>
                </div>
            </div>
            <div class="logo-nav-wrapper">
                <a href="/">
                    <img-ui
                        class="logo collapsing-phase"
                        path="entry/jig/logo-jigzi.svg"
                    ></img-ui>
                </a>
                <nav class="open-only">
                    <slot name="gallery"></slot>
                    ${this.isModulePage
                        ? nothing
                        : html`
                              <div class="divider"></div>
                              <slot name="modules"></slot>
                          `}
                </nav>
            </div>
            <div class="input open-only"><slot name="input"></slot></div>
            <div class="settings-preview">
                <slot name="settings"></slot>
                <div class="divider open-only"></div>
                <div class="open-only">
                    <slot name="preview"></slot>
                </div>
            </div>
        `;
    }
}
