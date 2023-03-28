import {
    LitElement,
    html,
    css,
    customElement,
    property,
    query,
} from "lit-element";
import { nothing } from "lit-html";
import { ModuleKind } from "@elements/module/_common/types";
import "@elements/core/images/ui";

export type ModuleState = "empty" | "active" | "thumbnail" | "unit";
const STR_EMPTY = "Drag\nactivity\nhere";
const STR_EMPTY_COVER = "Drag\ncover\nhere";

@customElement("jig-edit-sidebar-module-window")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .wrapper {
                    display: grid;
                    place-content: center;
                    width: 170px;
                    height: 96px;
                    border-radius: 16px;
                    box-sizing: border-box;
                }
                :host([state="empty"]) {
                    cursor: auto;
                }
                :host([state="empty"]) .wrapper {
                    background-color: var(--light-blue-5);
                    color: #ffffff;
                }
                :host([state="empty"]) .wrapper.drag-over {
                    background-color: var(--dark-blue-1);
                    color: transparent;
                }
                :host([state="empty"][coverOnly]) .wrapper:not(.drag-over) {
                    color: var(--dark-gray-3);
                    border: solid 3px var(--light-orange-2);
                    background-color: var(--light-orange-1);
                }
                :host([state="active"]) .wrapper {
                    border: solid var(--light-blue-5) 3px;
                    background-color: var(--white);
                }
                :host([state="unit"]) .wrapper {
                    border: 0 1px;
                    background-color: var(--light-blue-5);
                    overflow: hidden;
                }
                :host([incomplete]) .wrapper {
                    border: solid var(--light-red-4) 3px;
                }
                :host([state="active"]) img-ui {
                    height: 82px;
                }

                slot[name="thumbnail"] {
                    display: none;
                }

                slot[name="unit"] {
                    color: white;
                }

                :host([state="thumbnail"]) slot[name="thumbnail"] {
                    display: revert;
                }
                ::slotted([slot="thumbnail"]) {
                    border-radius: 16px;
                }

                :host([incomplete]) ::slotted([slot="thumbnail"]) {
                    border: none;
                }

                .drag-here-text {
                    white-space: pre-wrap;
                    font-size: 12px;
                    font-weight: bold;
                    text-align: center;
                    margin: 0;
                }
            `,
        ];
    }

    @property({ reflect: true })
    state: ModuleState = "active";

    @property()
    activeModuleKind: ModuleKind = "cover";

    @property({ type: Boolean, reflect: true })
    coverOnly: boolean = false;

    @property()
    publishedThumbnail: string = "";

    @property({ type: Boolean, reflect: true })
    incomplete: boolean = false;

    @query(".wrapper")
    wrapper!: HTMLElement;

    protected firstUpdated() {
        this.addEventListener("custom-drag-enter", () => {
            this.wrapper.classList.add("drag-over");
        });
        this.addEventListener("custom-drag-leave", _e => {
            this.wrapper.classList.remove("drag-over");
        });
        this.addEventListener("custom-drop", () => {
            this.wrapper.classList.remove("drag-over");
        });
    }

    // prettier-ignore
    render() {
        return html`
            <div class="wrapper">
                ${this.state === "empty"
                    ? html`
                          <!-- keep in one line -->
                          <p class="drag-here-text">${this.coverOnly ? STR_EMPTY_COVER : STR_EMPTY}</p>
                    `
                    : this.state === "active" ? html`
                        <img-ui
                            path="entry/jig/modules/large/${this
                                .activeModuleKind}-hover.svg"
                        ></img-ui>
                      `
                    : this.state === "thumbnail" ? html`
                        <slot name="thumbnail"></slot>
                    `
                    : this.state === "unit" ? html`
                        <slot name="unit"></slot>
                    `
                    : nothing
                }
            </div>
        `;
    }
}
