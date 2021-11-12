import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import "./jig-audio-playing-indicator";
import "@elements/core/images/ui";

const STR_NEW = "new";

@customElement("jig-audio-line")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    height: 48px;
                    align-items: center;
                    padding: 0 32px;
                }
                :host(:hover) {
                    background-color: var(--light-blue-2);
                }
                :host([playing]) {
                    background-color: var(--light-blue-2);
                }
                .start {
                    display: flex;
                    column-gap: 8px;
                    align-items: center;
                }
                .checkbox {
                    border: solid 1px #c7d3db;
                    height: 16px;
                    width: 16px;
                    font-size: 12px;
                    text-align: center;
                    line-height: 16px;
                    border-radius: 3px;
                    box-sizing: border-box;
                }
                :host([selected]) .checkbox {
                    background-color: var(--main-blue);
                    border: 0;
                    color: white;
                }
                ::slotted([slot="checkbox"]) {
                    border: solid 1px #c7d3db;
                    height: 16px;
                    width: 16px;
                    border-radius: 3px;
                    box-sizing: border-box;
                    cursor: pointer;
                }
                .name {
                    color: #4a4a4a;
                }
                .name .new {
                    display: none;
                    margin-left: 25px;
                    color: #42cc7a;
                }
                :host([new]) .name {
                    font-weight: bold;
                }
                :host([new]) .name .new {
                    display: inline-block;
                }
                .end {
                    display: flex;
                    column-gap: 24px;
                }
                .play-pause {
                    height: 24px;
                }
            `,
        ];
    }

    @property()
    label: string = "";

    @property({ type: Boolean, reflect: true })
    selected: boolean = false;

    @property({ type: Boolean, reflect: true })
    playing: boolean = false;

    @property({ type: Boolean, reflect: true })
    new: boolean = false;

    render() {
        return html`
            <div class="start">
                <slot name="checkbox"></slot>
                <div class="name">
                    ${this.label}
                    <span class="new">${STR_NEW}</span>
                </div>
            </div>
            <div class="end">
                ${this.playing
                    ? html`
                          <jig-audio-playing-indicator></jig-audio-playing-indicator>
                      `
                    : nothing}
                <div class="play-pause">
                    <slot name="play-pause"></slot>
                </div>
            </div>
        `;
    }
}
