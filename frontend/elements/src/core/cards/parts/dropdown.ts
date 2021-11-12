import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
@customElement("card-dropdown")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                main {
                    display: flex;
                    overflow: hidden;
                    scroll: auto;
                    justify-content: space-between;
                    width: 100%;
                    border-bottom: solid 1px #3c7df0;
                }
                .collapsed {
                    height: 44px;
                    overflow: hidden;
                }
                .expanded {
                    height: 100%;
                }
                .collapsed ::slotted([slot="content"]) {
                    display: none;
                }
                .content-wrapper {
                    display: flex;
                    flex-wrap: wrap;
                    width: 90%;
                }
                .pill {
                    height: 65px;
                }
                .inner-wrapper {
                    display: flex;
                    flex-wrap: wrap;
                }
                ::slotted([slot="content"]) {
                    margin-right: 14px;
                }
                ::slotted([slot="title"]) {
                    margin-bottom: 8px;
                    display: block;
                }
                .expanded img-ui {
                    transform: rotate(180deg);
                    align-self: end;
                }
            `,
        ];
    }

    @property({ type: Boolean })
    collapsed: boolean = false;

    @property({ type: Boolean })
    pill: boolean = false;

    @property()
    icon: string = "";

    @property()
    label: string = "";

    render() {
        const { collapsed, label, pill } = this;

        return html`
            <main
                class="${collapsed ? "collapsed" : "expanded"} ${pill
                    ? "pill"
                    : ""}"
            >
                <div class="content-wrapper">
                    <slot name="title">${label}</slot>

                    <div class="inner-wrapper">
                        <slot name="content"></slot>
                    </div>
                </div>

                <img-ui path="Icn_arrow_nm.svg"></img-ui>
            </main>
        `;
    }
}
