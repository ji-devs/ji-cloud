import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { createPopper, Placement, VirtualElement } from '@popperjs/core';
import { styleMap } from 'lit-html/directives/style-map';
import "@elements/core/buttons/icon";
import "./base";

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

    @property({type: Number})
    offsetSkidding:number = 0;

    @property({type: Number})
    offsetDistance:number = 24; //account for arrow

    @property()
    target:Element | VirtualElement | undefined;

    @property()
    placement:Placement = "right";

    @property()
    header:string = "";

    @property()
    confirmLabel:string = "";

    @property()
    cancelLabel:string = "";
    render() {
        const {header, confirmLabel, cancelLabel, target, maxWidth, placement, offsetSkidding, offsetDistance} = this;

        let bodyStyles:any = {
        };

        if(maxWidth !== -1) {
            bodyStyles.maxWidth = `${maxWidth}px`;
        }
        return html`

            <tooltip-base id="tooltip" color="red" .target=${target} .placement=${placement} .offsetSkidding=${offsetSkidding} .offsetDistance=${offsetDistance}>
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
