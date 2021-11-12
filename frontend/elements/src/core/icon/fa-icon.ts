import {
    LitElement,
    html,
    customElement,
    property,
    css,
    PropertyValues,
    internalProperty,
} from "lit-element";

// using pro-solid here but could technically use any other lib, just need the unicode char which is the same across libs (except brands)
import * as faIcons from "@fortawesome/pro-solid-svg-icons";

const ROOT = "https://ka-p.fontawesome.com/releases/v6.0.0-beta1";
const END = "?token=da13958c29";

function addMainCss() {
    const head = document.head;
    const link = document.createElement("link");

    link.type = "text/css";
    link.rel = "stylesheet";
    link.href = `${ROOT}/css/pro.min.css${END}`;

    head.appendChild(link);
}

addMainCss();

@customElement("fa-icon")
class _ extends LitElement {
    static get styles() {
        return [
            css`
                i {
                    font-family: "Font Awesome 6 Pro";
                    -webkit-font-smoothing: antialiased;
                    display: var(--fa-display, inline-block);
                    font-style: normal;
                    font-variant: normal;
                    line-height: 1;
                    text-rendering: auto;
                }
                :host([icon^="fa-solid"]) i {
                    font-weight: 900;
                }
                :host([icon^="fa-regular"]) i {
                    font-weight: 400;
                }
                :host([icon^="fa-light"]) i {
                    font-weight: 300;
                }
                :host([icon^="fa-thin"]) i {
                    font-weight: 100;
                }

                :host([icon^="fa-duotone"]) i {
                    position: relative;
                    font-family: "Font Awesome 6 Duotone";
                    font-weight: 900;
                }
                :host([icon^="fa-duotone"]) i::before {
                    position: absolute;
                    color: var(--fa-primary-color, inherit);
                    opacity: var(--fa-primary-opacity, 1);
                }
                :host([icon^="fa-duotone"]) i::after {
                    opacity: var(--fa-secondary-opacity, 0.4);
                }
            `,
        ];
    }

    private onIconChange() {
        const snakeCase = this.icon.split(" ")?.[1];

        if (!snakeCase) {
            this.iconPascal = "";
        } else {
            const words = snakeCase.split("-");
            for (let i = 1; i < words.length; i++) {
                words[i] =
                    words[i].slice(0, 1).toUpperCase() +
                    words[i].slice(1, words[i].length);
            }
            this.iconPascal = words.join("");
        }
    }

    @property({ reflect: true })
    icon = "";

    @internalProperty()
    private iconPascal = "";

    updated(propertyValues: PropertyValues) {
        if (propertyValues.has("icon")) {
            this.onIconChange();
        }
    }

    private getCode(icon: string) {
        return (faIcons as any)[icon]?.icon[3];
    }

    render() {
        const code = this.getCode(this.iconPascal);

        return html`
            ${this.icon.startsWith("fa-duotone")
                ? html`
                      <style>
                          i::before {
                              content: "\\${code}\\fe01";
                          }
                          i::after {
                              content: "\\${code}\\fe02";
                          }
                      </style>
                  `
                : html`
                      <style>
                          i::before {
                              content: "\\${code}";
                          }
                      </style>
                  `}

            <i class="${this.icon}"></i>
        `;
    }
}
