import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import { nothing } from "lit-html";

const STR_HEADER_FIRST = "Settings and JIG info.";
const STR_HEADER_SECOND = "Last step before publishing";
const STR_THUMBNAIL = "Thumbnail";

export type JigFocus = "modules" | "resources";

@customElement("jig-edit-publish")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    padding: 50px;
                    height: 100vh;
                    overflow: auto;
                    box-sizing: border-box;
                    overflow: auto;
                }
                .main-wrapper {
                    display: grid;
                    place-content: center;
                    min-height: 100%;
                }
                main {
                    display: grid;
                    place-content: center;
                    background-color: var(--white);
                    padding: 56px;
                    border-radius: 32px;
                    box-shadow: 0 3px 8px 0 rgba(0, 0, 0, 0.08);
                }
                .width-holder {
                    display: grid;
                    grid-template-rows: auto 1fr auto;
                    row-gap: 48px;
                    max-width: 1300px;
                }
                h1 {
                    font-size: 32px;
                    font-weight: 900;
                    color: var(--dark-blue-4);
                    margin: 0;
                }
                h3 {
                    font-weight: 500;
                    color: #4a4a4a;
                    margin: 0;
                }
                :host([jigFocus=resources]) h3 {
                    display: none;
                }
                .main {
                    display: grid;
                    grid-auto-columns: minmax(auto, 1fr);
                    grid-auto-flow: column;
                    column-gap: 48px;
                    justify-content: center;
                    align-items: start;
                }
                .img-wrapper {
                    display: grid;
                }
                .img-wrapper h4 {
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--main-blue);
                    grid-area: 1 / 1;
                    margin: 0;
                    transform: translateY(-2em);
                    padding-left: 16px;
                }
                ::slotted([slot="edit-cover"]) {
                    grid-column: 1;
                    grid-row: 1;
                    width: 32px;
                    height: 32px;
                    border-radius: 50%;
                    background-color: var(--main-blue);
                    color: white;
                    box-shadow: 0 0 4px #00000060;
                    justify-self: end;
                    display: inline-grid;
                    place-content: center;
                    transform: translate(50%, -50%);
                    cursor: pointer;
                }
                ::slotted([slot="img"]) {
                    grid-column: 1;
                    grid-row: 1;
                    display: grid;
                    width: 100%;
                    border-radius: 16px;
                    overflow: hidden;
                }
                ::slotted([slot="public"]) {
                    display: grid;
                    grid-template-columns: auto auto;
                    place-content: space-between;
                    margin-top: 24px;
                    padding: 0 16px;
                }
                .column-2 {
                    display: grid;
                    align-items: flex-start;
                    row-gap: 86px;
                }
                ::slotted([slot="description"]) {
                    height: 170px;
                }
                .column-3 {
                    display: grid;
                    align-items: flex-start;
                    row-gap: 40px;
                }
                .catagories {
                    display: grid;
                    row-gap: 16px;
                }
                ::slotted([slot="category-labels"]) {
                    display: flex;
                    flex-wrap: wrap;
                    column-gap: 8px;
                    row-gap: 12px;
                }
                .additional-resources {
                    display: grid;
                    row-gap: 16px;
                }
                .publish {
                    display: grid;
                    grid-auto-flow: column;
                    column-gap: 32px;
                    place-content: center;
                }
            `,
        ];
    }

    @property({ reflect: true })
    jigFocus: JigFocus = "modules";

    render() {
        return html`
            <div class="main-wrapper">
                <main>
                    <div class="width-holder">
                        <div class="header">
                            <h1>${STR_HEADER_FIRST}</h1>
                            <h3>${STR_HEADER_SECOND}</h3>
                            ${
                                this.jigFocus === "resources" ? html`
                                    <slot name="resources"></slot>
                                ` : nothing
                            }
                        </div>
                        <div class="main">
                            <div class="column-1">
                                <div class="img-wrapper">
                                    <h4>${STR_THUMBNAIL}</h4>
                                    <slot name="edit-cover"></slot>
                                    <slot name="img"></slot>
                                </div>
                                <div class="public">
                                    <slot name="public"></slot>
                                </div>
                            </div>
                            <div class="column-2">
                                <slot name="name"></slot>
                                <slot name="description"></slot>
                            </div>
                            <div class="column-3">
                                <slot name="language"></slot>
                                <slot name="age"></slot>
                                <slot name="goal"></slot>
                                <div class="catagories">
                                    <slot name="catagories-select"></slot>
                                    <slot name="category-labels"></slot>
                                </div>
                            </div>
                            ${
                                this.jigFocus === "modules" ? html`
                                    <div class="column-4 additional-resources">
                                        <slot name="resources"></slot>
                                    </div>
                                ` : nothing
                            }
                        </div>
                        <div class="publish">
                            <slot name="publish-later"></slot>
                            <slot name="publish"></slot>
                        </div>
                    </div>
                </main>
            </div>
        `;
    }
}
