import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { ThemeId, THEMES } from "@elements/_themes/themes";
import { themeIconPath } from "@elements/module/_groups/design/helpers";

const STR_SAMPLE_HEBREW = "אבא";
const STR_SAMPLE_ENGLISH = "Mom";

@customElement("theme-selector-design-option")
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
                    top: 0;
                    left: 0;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    margin-top: 20px;
                }
                @media (min-width: 1920px) {
                    .content {
                        margin-top: 30px;
                    }
                }

                img-ui {
                    margin-bottom: 14px;
                    width: 136px;
                }
                @media (min-width: 1920px) {
                    img-ui {
                        margin-bottom: 16px;
                        width: 200px;
                    }
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
                    <img-ui
                        class=${imageClass}
                        path="${themeIconPath(theme, false)}"
                    ></img-ui>
                    <img-ui
                        class=${imageHoverClass}
                        path="${themeIconPath(theme, true)}"
                    ></img-ui>
                    <div class="label">${THEMES[theme].label.en}</div>
                </div>
            </section>
        `;
    }
}
