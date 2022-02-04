import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { ThemeId, THEMES } from "@elements/_themes/themes";
import { cardBackIconPath } from "@elements/module/_groups/cards/helpers";

@customElement("theme-selector-cards-option")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    cursor: pointer;
                }
                section {
                    position: relative;
                    border-radius: 16px;
                    border: solid 3px rgba(0, 0, 0, 0);
                    box-sizing: border-box;
                    width: 168px;
                    height: 150px;
                }
                @media (min-width: 1920px) {
                    section {
                        width: 232px;
                        height: 196px;
                    }
                }

                section.hover {
                    border: solid 3px var(--light-blue-4);
                }

                :host([selected]) section {
                    border: solid 3px var(--light-blue-4);
                    background-color: var(--light-blue-3);
                }

                .content {
                    position: relative;
                    top: 0px;
                    left: 0px;
                }
                .left,
                .right {
                    box-sizing: border-box;
                    width: 76px;
                    height: 76px;
                }
                @media (min-width: 1920px) {
                    .left,
                    .right {
                        width: 112px;
                        height: 112px;
                    }
                }
                .left {
                    position: absolute;
                    top: 16px;
                    left: 16px;
                    border-radius: 16px;
                    box-shadow: 0px 3px 6px rgba(0, 0, 0, 0.16);
                }

                .right {
                    position: absolute;
                    top: 28px;
                    left: calc(54px + 13px);
                }
                @media (min-width: 1920px) {
                    .right {
                        top: 32px;
                        left: calc(88px + 13px);
                    }
                }

                .right {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }

                .menu {
                    position: absolute;
                    top: -16px;
                    right: -16px;
                    z-index: 1;
                }

                .label {
                    position: absolute;
                    left: 0px;
                    text-align: center;
                    width: 100%;
                    font-size: 14px;
                    font-weight: 500;
                    color: var(--dark-blue-8);
                    top: 114px;
                }
                @media (min-width: 1920px) {
                    .label {
                        top: 160px;
                    }
                }

                .hidden {
                    display: none;
                }

                :host([selected]) .label {
                    color: var(--main-blue);
                }

                img-ui {
                    width: 100%;
                    height: 100%;
                    object-fit: cover;
                }
            `,
        ];
    }

    @property()
    theme: ThemeId = "blank";

    @property({ type: Boolean, reflect: true })
    selected: boolean = false;

    @property({ type: Boolean })
    hover: boolean = false;

    onEnter() {
        this.hover = true;
    }

    onLeave() {
        this.hover = false;
    }

    render() {
        const { theme, hover } = this;

        const sectionClasses = classMap({
            hover,
        });

        const imageClass = classMap({
            hidden: hover,
        });
        const imageHoverClass = classMap({
            hidden: !hover,
        });
        return html`
            <section
                class=${sectionClasses}
                @mouseenter="${this.onEnter.bind(this)}"
                @mouseleave="${this.onLeave.bind(this)}"
            >
                <div class="content">
                    <div class="right">
                        <img-ui path="${cardBackIconPath(theme)}"></img-ui>
                    </div>
                    <div class="left">
                        <img-ui
                            class=${imageClass}
                            path="theme/${theme}/card-front-icon.svg"
                        ></img-ui>
                        <img-ui
                            class=${imageHoverClass}
                            path="theme/${theme}/card-front-icon-hover.svg"
                        ></img-ui>
                    </div>
                    <div class="label">${THEMES[theme].label.en}</div>
                </div>
            </section>
        `;
    }
}
