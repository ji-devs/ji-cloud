import { mediaUi } from "@utils/path";
import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";

@customElement("jig-play-sidebar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 416px auto;
                    justify-content: start;
                    transform: translateX(-416px);
                    transition: transform .3s;
                    height: 100%;
                }
                :host([open]) {
                    transform: translateX(0px);
                }
                ::slotted([slot=opener]) {
                    background-color: transparent;
                    border: 0;
                    padding: 0;
                    grid-column: 2;
                    align-self: start;
                    margin-top: 80px;
                    height: 120px;
                    width: 120px;
                    cursor: pointer;
                    margin-left: -40px;
                    transition: transform .2s, opacity .2s;
                    transform-origin: left top;
                    border-radius: 50%;
                    background-image: url(${unsafeCSS(mediaUi("entry/jig/play/sidebar/opener.svg"))});
                    background-position: center;
                    background-color: transparent;
                    box-shadow: 0 3px 6px 0 rgb(0 0 0 / 16%);
                }
                :host([open]) ::slotted([slot=opener]) {
                    opacity: 0;
                    pointer-events: none;
                }
                ::slotted([slot=opener]:hover) {
                    transform: rotate(-20deg);
                }
                main {
                    grid-row: 1;
                    grid-column: 1;
                    display: grid;
                    grid-template-rows: auto 1fr;
                    width: 416px;
                    box-shadow: 0 3px 20px 0 rgba(0, 0, 0, 0.08);
                    background-color: #fff;
                    z-index: 1;
                }
                h2 {
                    color: #3558af;
                    font-size: 22px;
                    font-weight: bold;
                    margin: 0;
                }
                .heading {
                    border-bottom: solid 1px #e2e5eb;
                    padding: 20px 24px;
                    display: grid;
                    row-gap: 14px;
                }
                .bottom {
                    display: grid;
                    align-items: start;
                }
                .actions {
                    grid-column: 1;
                    grid-row: 1;
                    padding: 16px 24px;
                    display: flex;
                    column-gap: 16px;
                    z-index: 2;
                }
                ::slotted(*)::part(overlay) {
                    z-index: 2;
                    background-color: green
                }
                ::slotted(::part(overlay)) {
                    z-index: 2;
                    background-color: red
                }
                .modules {
                    grid-column: 1;
                    grid-row: 1;
                    padding-top: 210px;
                    overflow-y: auto;
                    scrollbar-width: thin;
                    scrollbar-color: var(--light-gray-1) transparent;
                }
                .modules::-webkit-scrollbar-track {
                    background-color: #fff;
                }
                .modules::-webkit-scrollbar {
                    width: 8px;
                }
                .modules::-webkit-scrollbar-thumb {
                    border-radius: 4px;
                    background-color: var(--light-gray-1);
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    @property()
    jigName: string = "";

    render() {
        return html`
            <slot name="opener"></slot>
            <main>
                <div class="heading">
                    <img-ui path="entry/jig/logo-jigzi.svg"></img-ui>
                    <h2>${this.jigName}</h2>
                </div>
                <div class="bottom">
                    <div class="actions">
                        <slot name="actions"></slot>
                    </div>
                    <div class="modules">
                        <slot name="modules"></slot>
                    </div>
                </div>
            </main>
        `;
    }
}
