import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import "@elements/core/buttons/icon";
import { collapseStyles } from "../../_common/sidebar-modules/collapse-styles";
import { AssetType } from "@elements/module/_common/types";


@customElement("jig-edit-sidebar")
export class _ extends LitElement {
    static get styles() {
        return [
            collapseStyles,
            css`
                :host {
                    display: block;
                    width: 56px;
                    height: 100%;
                }
                :host([isModulePage]) {
                    width: 324px;
                }
                .page-overlay {
                    position: fixed;
                    width: 100vw;
                    height: 100dvh;
                    display: grid;
                    background-color: #ffffff80;
                    transition-property: background-color;
                }
                :host([isModulePage]) .page-overlay {
                    display: none;
                }
                :host([collapsed]) .page-overlay,
                :host([isModulePage]) .page-overlay {
                    background-color: #ffffff00;
                    pointer-events: none;
                }
                .sidebar-content {
                    position: absolute;
                    display: flex;
                    flex-flow: column;
                    height: 100dvh;
                    width: 324px;
                    box-shadow: 0 3px 20px 0 rgba(0, 0, 0, 0.08);
                    background-color: var(--white);
                    transition-property: width;
                    transition-timing-function: linear;
                    transition-delay: 0s;
                    transition-duration: var(--collapsing-phase-duration);
                    background-color: #ffffff;
                }
                :host([collapsed]) .sidebar-content {
                    width: 56px;
                    transition-delay: var(--fading-phase-duration);
                }
                .side-head {
                    opacity: 0;
                    transition-property: opacity;
                    height: 0;
                }
                :host([collapsed]) .side-head {
                    opacity: 1;
                }
                section {
                    margin-top: 7px;
                    transition-property: margin-top;
                    height: 100%;
                    overflow-y: auto;
                    overflow-x: hidden;
                    scrollbar-width: thin;
                    scrollbar-color: #d3d4dd transparent;
                }
                section::-webkit-scrollbar-track {
                    background-color: #fff;
                }
                section::-webkit-scrollbar {
                    width: 6px;
                }
                section::-webkit-scrollbar-thumb {
                    border-radius: 3px;
                    background-color: #d3d4dd;
                }
                :host([collapsed]) section {
                    margin-top: 100px;
                }
                .cover-module,
                :host([collapsed]) .cover-module {
                    margin-top: 2px;
                    transition-property: margin-top;
                }
                .publish {
                    /* Allow room for jiggling feet */
                    margin-bottom: 84px;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    collapsed: boolean = false;

    @property({ type: Boolean, reflect: true })
    isModulePage: boolean = false;

    @property()
    assetType: AssetType | "" = "";

    render() {
        const coverModuleMarginTop = this.assetType === "course" ? "0px" : "180px";

        return html`
            <div class="page-overlay collapsing-phase"></div>
            <div class="sidebar-content">
                <header>
                    <slot name="header"></slot>
                </header>
                <div class="side-head collapsing-phase">
                    <slot name="side-head"></slot>
                </div>
                <section class="collapsing-phase">
                    <div class="cover-module collapsing-phase" style="margin-top: ${coverModuleMarginTop}">
                        <slot name="cover-module"></slot>
                    </div>
                    <div class="modules">
                        <slot name="modules"></slot>
                    </div>
                    <div class="publish">
                        <slot name="publish"></slot>
                    </div>
                </section>
            </div>
        `;
    }
}
