import {
    LitElement,
    html,
    css,
    customElement,
    property,
    internalProperty,
} from "lit-element";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import "@elements/core/menu/kebab";
import "@elements/core/menu/menu-line";
import { styles } from "./styles";
import { classMap } from "lit-html/directives/class-map";

const STR_DRAFT = "Draft";

@customElement("jig-gallery-recent")
export class _ extends LitElement {
    static get styles() {
        return [
            styles,
            css`
                a {
                    text-decoration: none;
                    color: var(--dark-gray-5);
                }
                .draft {
                    padding: 6px 8px;
                    border-radius: 6px;
                    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.16);
                    background-color: #ffea79;
                    grid-column: 2;
                    height: 32px;
                    font-size: 14px;
                    font-weight: 600;
                    line-height: 1.5;
                    box-sizing: border-box;
                    margin-top: -8px;
                    margin-left: 16px;
                    position: absolute;
                    display: inline-flex;
                    align-items: center;
                    column-gap: 4px;
                }
                .card {
                    display: inline-grid;
                    background-color: #fff;
                    height: 230px;
                    grid-template-rows: 130px auto;
                }
                .top-section {
                    display: grid;
                    grid-template-rows: 16px auto 1fr;
                    grid-template-columns: 1fr auto 16px;
                }
                .top-section ::slotted([slot="thumbnail"]) {
                    grid-column: 1 / -1;
                    grid-row: 1 / -1;
                    border-radius: 16px 16px 0 0;
                }
                .menu {
                    display: none;
                    grid-column: 2;
                    grid-row: 2;
                }
                :host(:hover) .menu,
                .menu.menu-open {
                    display: block;
                }
                .bottom-section {
                    padding: 16px;
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    align-content: space-between;
                }
                .label {
                    grid-column: 1 / -1;
                    text-align: center;
                }
                .last-edited {
                    font-size: 14px;
                    font-weight: 500;
                    text-align: right;
                }
            `,
        ];
    }

    @property()
    label: string = "";

    @property({ type: Boolean, reflect: true })
    draft = false;

    @property()
    ages: string = "";

    @property()
    publishedAt: string = "";

    @property()
    href: string = "";

    @internalProperty()
    menuOpen = false;

    render() {
        return html`
            <a href="${this.href}">
                ${this.draft
                    ? html`<div class="draft">
                          <img-ui
                              path="entry/jig/gallery/draft-icon.svg"
                          ></img-ui>
                          <span>${STR_DRAFT}</span>
                      </div>`
                    : nothing}
                <div class="card">
                    <div class="top-section">
                        <slot name="thumbnail"></slot>
                        <menu-kebab
                            class="menu ${classMap({
                                "menu-open": this.menuOpen,
                            })}"
                            @open="${() => (this.menuOpen = true)}"
                            @close="${() => (this.menuOpen = false)}"
                            @click="${(e: Event) => e.preventDefault()}"
                        >
                            <slot name="menu-content"></slot>
                        </menu-kebab>
                    </div>
                    <div class="bottom-section">
                        <span class="label main-text">${this.label}</span>
                        <span class="ages">
                            <img-ui
                                path="entry/jig/gallery/age-icon${this.draft
                                    ? "-draft"
                                    : ""}.svg"
                            ></img-ui>
                            ${this.ages}
                        </span>
                        <span class="last-edited">${this.publishedAt}</span>
                    </div>
                </div>
            </a>
        `;
    }
}
