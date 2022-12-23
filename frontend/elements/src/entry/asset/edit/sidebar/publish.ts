import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import "@elements/core/buttons/icon";
import { collapseStyles } from "../../_common/sidebar-modules/collapse-styles";

const STR_END = "End";
const STR_PUBLISH = "Publish";

@customElement("jig-edit-sidebar-publish")
export class _ extends LitElement {
    static get styles() {
        return [
            collapseStyles,
            css`
                section {
                    display: flex;
                    cursor: pointer;
                    box-sizing: border-box;
                    border: solid 1px transparent;
                    border-left: solid 4px transparent;
                    border-bottom: solid 2px transparent;
                    width: 324px;
                    height: 136px;
                    transition-property: height, width;
                    transition-duration: var(--collapsing-phase-duration);
                    transition-timing-function: linear;
                }
                :host([selected]) section {
                    border-color: #e7f0fd;
                    background-color: #f8f9fd;
                    border-left-color: var(--main-blue);
                }
                :host([collapsed]) section {
                    height: 106px;
                    width: 58px;
                    transition-delay: var(--fading-phase-duration);
                    border-bottom-color: #e7f0fd;
                }
                .grid-container {
                    margin-top: 18px;
                    display: grid;
                    grid-template-columns: 98px auto auto;
                }
                .left {
                    padding-left: 14px;
                    display: flex;
                    flex-direction: column;
                    text-align: center;
                    width: 0;
                    opacity: 0;
                    transition-property: opacity;
                    transition-duration: var(--collapsing-phase-duration);
                    transition-timing-function: linear;
                }
                :host([collapsed]) .left {
                    width: 50px;
                    padding-left: 0;
                    transition-delay: var(--fading-phase-duration);
                    opacity: 1;
                }
                .middle {
                    width: 168px;
                }
                .right {
                    margin-left: 5px;
                    display: flex;
                    z-index: 1;
                }

                .side-title {
                    font-size: 14px;
                    font-weight: bold;
                    line-height: 1.5;
                    color: var(--main-blue);
                }
                .icon {
                    margin-top: 8px;
                }
                .icon img-ui {
                    width: 28px;
                }
                .window {
                    position: relative;
                    z-index: 1;
                    width: 130px;
                    height: 100px;
                    border-radius: 62px;
                    border: solid 2px var(--light-blue-3);
                    background-color: var(--light-blue-2);
                    margin: 0 auto;
                    display: flex;
                    justify-content: center;
                    flex-direction: column;
                    align-items: center;
                }
                .window-icon {
                    height: 32px;
                    width: 32px;
                }
                .window-title {
                    font-size: 12px;
                    font-weight: bold;
                    color: var(--main-blue);
                }
                :host([published]) .window-title {
                    color: #42cc7a;
                }
                .decorations {
                    position: relative;
                    top: 0;
                    left: 0;
                    pointer-events: none;
                    scale: 75%;
                    translate: -18px;
                }
                .feet-spring,
                .feet-rollers {
                    position: absolute;
                    top: 0;
                    left: 0;
                }
                .feet-spring {
                    transform: translate(92px, 90px);
                }
                .feet-rollers {
                    transform: translate(49px, 162px);
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    selected: boolean = false;

    @property({ type: Boolean, reflect: true })
    published: boolean = false;

    @property({ type: Boolean, reflect: true })
    collapsed: boolean = true;

    render() {
        return html`
            <section>
                <div class="grid-container">
                    <div class="left">
                        <div class="side-title">${STR_END}</div>
                        <div class="icon">
                            <img-ui
                                path="entry/jig/modules/small/publish.svg"
                            ></img-ui>
                        </div>
                    </div>
                    <div class="middle open-only">
                        <div class="decorations">
                            <img-ui
                                class="feet-spring"
                                path="entry/jig/jiggling/feet-spring.svg"
                            ></img-ui>
                            <img-ui
                                class="feet-rollers"
                                path="entry/jig/jiggling/yellow/feet-rollers.svg"
                            ></img-ui>
                        </div>
                        <div class="window">
                            <img-ui
                                class="window-icon"
                                path="entry/jig/modules/small/publish-${this.published ? "green" : "blue"}-bg.svg"
                            ></img-ui>
                            <span class="window-title">${STR_PUBLISH}</span>
                        </div>
                    </div>
                    <div class="right open-only">
                        <div class="menu">
                            <slot name="menu"></slot>
                        </div>
                    </div>
                </div>
            </section>
        `;
    }
}
