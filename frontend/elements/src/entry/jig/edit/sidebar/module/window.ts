import { LitElement, html, css, customElement, property, query } from "lit-element";
import { nothing } from "lit-html";
import { ModuleKind, moduleKinds } from "@elements/entry/jig/module-types";
import "@elements/core/images/ui";


export type ModuleState = "empty" | "draft" | "active" | "complete" | "published";
const STR_EMPTY = "Drag\nactivity\nhere"

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
                :host([state=empty]) .wrapper {
                    background-color: var(--light-blue-5);
                }
                :host([state=empty]) .wrapper.drag-over {
                    background-color: var(--dark-blue-1);
                }
                :host([state=draft]) .wrapper {
                    background-color: var(--light-blue-2);
                    border: solid 2px #d8e7f9;
                }
                :host([state=active]) .wrapper {
                    background-color: var(--light-blue-5);
                }
                :host([state=active]) img-ui {
                    height: 100px;
                }
                :host([state=complete]) {
                    background-color: #d5f0de;
                    border: solid 2px #c5e9d2;
                }
                :host([state=published]) {
                    
                }

                .drag-here-text {
                    color: white;
                    white-space: pre-wrap;
                    font-size: 14px;
                    font-weight: bold;
                    font-stretch: normal;
                    font-style: normal;
                    line-height: 1.29;
                    letter-spacing: normal;
                    text-align: center;
                    color: var(--white);
                    margin: 0;
                }
                .drag-over .drag-here-text {
                    color: transparent;
                }
            `,
        ];
    }

    @property({reflect: true})
    state: ModuleState = "draft";

    @property()
    activeModuleKind: ModuleKind = "memory";

    @property()
    publishedThumbnail: string = "";

    @query(".wrapper")
    wrapper!: HTMLElement;

    dragOver() {
        this.wrapper.classList.add("drag-over");
    }
    dragLeave() {
        this.wrapper.classList.remove("drag-over");
    }

    render() {
        return html`
            <div class="wrapper" @dragover="${this.dragOver}" @dragleave="${this.dragLeave}">
                ${
                    this.state === "empty" ? html`
                        <p class="drag-here-text">${STR_EMPTY}</p>
                    `: this.state === "draft" ? html`
                        <img-ui path="core/buttons/icon/circle-pencil-blue.svg"></img-ui>
                    ` : this.state === "active" ? html`
                        <img-ui path="entry/jig/modules/large/${this.activeModuleKind}-hover.svg"></img-ui>
                    ` : this.state === "complete" ? html`
                        <img-ui path="core/buttons/icon/circle-check-green.svg"></img-ui>
                    ` : this.state === "published" ? html`
                        TODO: <img-ji></img-ji>
                    `: nothing
                }
            </div>
        `;
    }
}
