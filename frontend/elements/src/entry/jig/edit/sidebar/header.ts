import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import "@elements/core/inputs/text-pencil";
import "@elements/core/buttons/icon";
import "@elements/core/buttons/text";
import { collapseStyles } from "./collapse-styles";


@customElement("jig-edit-sidebar-header")
export class _ extends LitElement {
    static get styles() {
        return [
            collapseStyles,
            css`
                :host {
                    padding: 20px;
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
                    margin-right: -10px;
                }
                .close {
                    height: 20px;
                    width: 20px;
                    cursor: pointer;
                    border-radius: 50%;
                    display: inline-grid;
                    place-content: center;
                }
                .close:hover, .close:active {
                    background-color: var(--main-blue);
                }
                :host([isModulePage]) .close {
                    opacity: 0;
                    cursor: inherit;
                }
                :host([collapsed]) .close {
                    margin-right: 0px;
                    transition-property: margin-right;
                }
                :host([collapsed]) ::slotted([slot=close]) {
                    transform: rotate(-180deg);
                }
                ::slotted([slot=close]) {
                    transition: transform .3s;
                    width: 14px;
                    height: 14px;
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
                nav {
                    display: flex;
                    align-items: center;
                    column-gap: 12px;
                }
                nav ::slotted([slot=settings]) {
                    height: 32px;
                }
                /* will probably need to be replaced */
                /* nav ::slotted(img-ui) { */
                nav ::slotted([slot=settings]), nav ::slotted([slot=modules]) {
                    filter: opacity(0.6);
                }
                .input {
                    margin-top: 23px;
                    width: 100%;
                }
            `,
        ];
    }

    @property({type: Boolean, reflect: true})
    collapsed: boolean = false;

    @property({type: Boolean, reflect: true})
    isModulePage: boolean = false;

    render() {
        return html`
            <div class="close-wrapper">
                <div class="close collapsing-phase">
                    <slot name="close"></slot>
                </div>
            </div>
            <div class="logo-nav-wrapper">
                <img-ui class="logo collapsing-phase" path="entry/jig/logo-jigzi.svg"></img-ui>
                <nav class="open-only">
                    <slot name="gallery"></slot>
                    <div class="divider"></div>
                    <slot name="settings"></slot>
                    ${ this.isModulePage ? nothing : (html`
                        <div class="divider"></div>
                        <slot name="modules"></slot>
                    `) }
                </nav>
            </div>
            <div class="input open-only"><slot name="input"></slot></div>
            <div class="preview open-only"><slot name="preview"></slot></div>
        `;
    }
}
