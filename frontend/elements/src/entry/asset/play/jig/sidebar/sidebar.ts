import { mediaUi } from "@utils/path";
import {
    LitElement,
    html,
    css,
    customElement,
    property,
    unsafeCSS,
} from "lit-element";

@customElement("jig-play-sidebar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 316px auto;
                    justify-content: start;
                    transform: translateX(-324px);
                    transition: transform 0.3s;
                    height: 100%;
                }
                :host([open]) {
                    transform: translateX(0px);
                }
                ::slotted([slot="opener"]) {
                    border: 0;
                    padding: 0;
                    grid-column: 2;
                    align-self: start;
                    margin-top: 60px;
                    height: 90px;
                    width: 90px;
                    cursor: pointer;
                    margin-left: -26px;
                    transition: transform 0.2s, opacity 0.2s;
                    transform-origin: left top;
                    border-radius: 50%;
                    background-image: url(${unsafeCSS(
                        mediaUi("entry/jig/play/sidebar/opener.svg")
                    )});
                    background-position: center;
                    background-color: transparent;
                    background-size: 118px;
                    box-shadow: 0 3px 6px 0 rgb(0 0 0 / 16%);
                }
                :host([open]) ::slotted([slot="opener"]) {
                    opacity: 0;
                    pointer-events: none;
                }
                ::slotted([slot="opener"]:hover) {
                    transform: rotate(-20deg);
                }
                main {
                    grid-row: 1;
                    grid-column: 1;
                    display: grid;
                    grid-template-rows: auto 1fr;
                    width: 324px;
                    height: 100dvh;
                    box-shadow: 0 3px 20px 0 rgba(0, 0, 0, 0.08);
                    background-color: #fff;
                    z-index: 1;
                }
                h2 {
                    color: #3558af;
                    font-size: 20px;
                    font-weight: bold;
                    margin: 0;
                }
                .heading {
                    border-bottom: solid 1px #e2e5eb;
                    padding: 16px 20px;
                    display: grid;
                    row-gap: 12px;
                    grid-template-columns: 1fr auto;
                }
                .logo {
                    place-self: start;
                }
                .logo img-ui {
                    height: 30px;
                }
                ::slotted([slot="close"]) {
                    align-self: start;
                    height: 32px;
                    width: 32px;
                    display: grid;
                    place-content: center;
                    margin: -18px -18px 0px 0px;
                    color: #4a4a4a;
                    font-size: 24px;
                    font-weight: 300;
                }
                .bottom {
                    display: grid;
                    overflow: hidden;
                }
                .actions {
                    grid-column: 1;
                    grid-row: 1;
                    padding: 12px 20px;
                    display: flex;
                    column-gap: 16px;
                    z-index: 2;
                    align-self: start;
                }
                ::slotted(*)::part(overlay) {
                    z-index: 2;
                    background-color: green;
                }
                ::slotted(::part(overlay)) {
                    z-index: 2;
                    background-color: red;
                }
                .modules {
                    grid-column: 1;
                    grid-row: 1;
                    padding: 184px 0 60px 0;
                    overflow-y: auto;
                    scrollbar-width: thin;
                    scrollbar-color: var(--light-gray-1) transparent;
                }
                .modules::-webkit-scrollbar-track {
                    background-color: #fff;
                }
                .modules::-webkit-scrollbar {
                    width: 6px;
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
                    <a class="logo" href="/" target="_top">
                        <img-ui path="entry/jig/logo-jigzi.svg"></img-ui>
                    </a>
                    <slot name="close"></slot>
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
