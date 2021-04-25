import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { styleMap } from 'lit-html/directives/style-map';
import "@elements/core/buttons/icon";
import "./base";
import {COLOR, Placement, ElementTarget} from "./base";

@customElement("tooltip-confirm")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                font-family: Poppins;
            }
            .body {
                font-size: 16px;
                color: var(--dark-gray-6);
            }

            .buttons {
                display: flex;
                margin-top: 37px;
                gap: 31px;
                align-items: center;
            }

            .buttons > * {
                cursor: pointer;
            }

            .confirm {
                color: var(--red-alert);
            }

            .cancel {
                border: solid 1px #2a68d2;
                color: var(--dark-blue-2);
                border-radius: 16px;
                padding: 5px 15px;
            }
            article {
                display: flex;
                gap: 16px;
            }
            `
        ];
    }


    connectedCallback() {
        super.connectedCallback();
        window.addEventListener("mousedown", this.onGlobalMouseDown);
    }
    disconnectedCallback() {
        super.disconnectedCallback();
        window.removeEventListener("mousedown", this.onGlobalMouseDown);
    }

    onConfirm = () => {
        this.dispatchEvent(new Event("accept"))
    }
    onCancel = () => {
        this.dispatchEvent(new Event("close"))
    }
    onGlobalMouseDown = (evt: MouseEvent) => {
        if(!evt.composedPath().includes(this.shadowRoot?.getElementById("tooltip") as any)) {
            this.onCancel();
        }
    }

    @property({type: Number})
    maxWidth:number = -1;

    @property()
    header:string = "";

    @property()
    confirmLabel:string = "";

    @property()
    cancelLabel:string = "";

    //pass through
    @property({reflect: true})
    color:COLOR = "red";

    @property()
    target:ElementTarget | undefined;

    @property()
    placement:Placement = "left";

    @property({type: Number})
    margin:number = 0;

    @property({type: Number})
    arrowOffset:number = 0;

    render() {
        const {header, confirmLabel, cancelLabel, target, maxWidth, placement, color, arrowOffset, margin} = this;

        let bodyStyles:any = {
        };

        if(maxWidth !== -1) {
            bodyStyles.maxWidth = `${maxWidth}px`;
        }
        return html`

            <tooltip-base id="tooltip" color=${color} .target=${target} .placement=${placement} margin=${margin} arrowOffset=${arrowOffset}>
                <article>
                    <img-ui path="core/tooltips/alert.svg"></img-ui>
                    <div class="body" style="${styleMap(bodyStyles)}">
                        <div class="header">${header}</div>
                        <div class="buttons">
                            <div class="confirm" @click=${this.onConfirm} >${confirmLabel}</div>
                            <div class="cancel" @click=${this.onCancel} >${cancelLabel}</div>
                        </div>
                    </div>
                </article>
            </tooltip-base>

        `;
    }
}
