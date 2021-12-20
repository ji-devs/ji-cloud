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

export type ModuleState = "empty" | "active" | "thumbnail";
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
                    width: 218px;
                    height: 123px;
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
                    background-color: var(--light-blue-5);
                }
                :host([state="active"]) img-ui {
                    height: 100px;
                }
                slot[name="thumbnail"] {
                    display: none;
                }
                :host([state="thumbnail"]) slot[name="thumbnail"] {
                    display: revert;
                }
                ::slotted([slot="thumbnail"]) {
                    border-radius: 16px;
                }

                .drag-here-text {
                    white-space: pre-wrap;
                    font-size: 14px;
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

    @query(".wrapper")
    wrapper!: HTMLElement;

    addDragClass() {
        this.wrapper.classList.add("drag-over");
    }
    removeDragClass() {
        this.wrapper.classList.remove("drag-over");
    }

    // prettier-ignore
    render() {
        return html`
            <div
                class="wrapper"
                @dragover="${this.addDragClass}"
                @dragleave="${this.removeDragClass}"
                @drop="${this.removeDragClass}"
            >
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
                    : nothing
                }
            </div>
        `;
    }
}
