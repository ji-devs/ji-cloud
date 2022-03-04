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
                    grid-template-columns: 416px auto;
                    justify-content: start;
                    transform: translateX(-424px);
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
                    margin-top: 80px;
                    height: 120px;
                    width: 120px;
                    cursor: pointer;
                    margin-left: -40px;
                    transition: transform 0.2s, opacity 0.2s;
                    transform-origin: left top;
                    border-radius: 50%;
                    background-image: url(${unsafeCSS(
                        mediaUi("entry/jig/play/sidebar/opener.svg")
                    )});
                    background-position: center;
                    background-color: transparent;
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
                    width: 424px;
                    height: 100vh;
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
                    grid-template-columns: 1fr auto;
                }
                .logo {
                    place-self: start;
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
                    padding: 16px 24px;
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

                /* mobile */
                @media (max-width: 1000px) {
                    ::slotted([slot="opener"]) {
                        margin-top: 10px;
                        height: 60px;
                        width: 60px;
                        margin-left: -10px;
                        background-size: 80px;
                    }
                    h2 {
                        font-size: 16px;
                    }
                    .heading {
                        padding: 4px 6px;
                        row-gap: 0;
                    }
                    .logo img-ui {
                        height: 24px;
                    }
                    ::slotted([slot="close"]) {
                        margin: -8px -12px 0px 0px;
                    }
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
