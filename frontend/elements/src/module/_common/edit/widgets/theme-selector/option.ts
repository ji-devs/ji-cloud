import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { ThemeId, THEMES } from "@elements/_themes/themes";
import {nothing} from "lit-html";

const themeIconPath = (theme: ThemeId, hover: boolean, optionType?: String): string => {
    return `theme/${theme}/icon${optionType ? `-${optionType}` : ""}${hover ? "-hover" : ""}.jpg`;
};

@customElement("theme-selector-option")
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
                    width: 136px;
                    height: 120px;
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
                    top: 0;
                    left: 0;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    margin-top: 16px;
                }

                .content > div {
                    position: relative;
                }

                .content .premium {
                    padding: 0 4px;
                    border-radius: 2px;
                    border: solid 1px var(--light-blue-3);
                    background-color: var(--white);
                    position: absolute;
                    left: 6px;
                    bottom: 12px;
                }

                .content .premium img-ui {
                    width: 11px;
                }

                .content .premium .premium-label {
                    display: none;
                    font-family: 'Poppins', sans-serif;
                    font-size: 11px;
                    font-weight: 600;
                    margin-left: 6px;
                }

                .content .premium:hover .premium-label {
                    display: inline-block;
                }

                img-ui {
                    margin-bottom: 12px;
                    width: 106px;
                }

                .menu {
                    position: absolute;
                    top: -16px;
                    right: 0px;
                    z-index: 1;
                }

                .hidden {
                    display: none;
                }
                .label {
                    text-align: center;
                    font-size: 14px;
                    font-weight: 500;
                    color: var(--dark-blue-8);
                }

                :host([selected]) .label {
                    color: var(--main-blue);
                }
            `,
        ];
    }

    @property()
    theme: ThemeId = "blank";

    @property({ type: Boolean, reflect: true })
    selected: boolean = false;

    @property({ type: Boolean, reflect: true })
    hover: boolean = false;

    @property({ type: String })
    optionType?: String;

    @property({ type: Boolean })
    premium: boolean = false;

    onEnter() {
        this.hover = true;
    }

    onLeave() {
        this.hover = false;
    }

    render() {
        const { theme, optionType, hover } = this;

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
                    <div>
                        ${this.premium
                            ? html`<span class="premium"><img-ui
                                class="jiggling"
                                path="icons/pro-icon-small.svg"
                            ></img-ui><span class="premium-label">Pro</span></span>`
                            : nothing
                        }
                        <img-ui
                            class=${imageClass}
                            path="${themeIconPath(theme, false, optionType)}"
                        ></img-ui>
                        <img-ui
                            class=${imageHoverClass}
                            path="${themeIconPath(theme, true, optionType)}"
                        ></img-ui>
                    </div>
                    <div class="label">${THEMES[theme].label.en}</div>
                </div>
            </section>
        `;
    }
}
