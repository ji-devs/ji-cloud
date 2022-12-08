import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { ThemeId, THEMES } from "@elements/_themes/themes";

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
                    <img-ui
                        class=${imageClass}
                        path="${themeIconPath(theme, false, optionType)}"
                    ></img-ui>
                    <img-ui
                        class=${imageHoverClass}
                        path="${themeIconPath(theme, true, optionType)}"
                    ></img-ui>
                    <div class="label">${THEMES[theme].label.en}</div>
                </div>
            </section>
        `;
    }
}
