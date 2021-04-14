import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { createPopper, Placement, VirtualElement } from '@popperjs/core';
import { styleMap } from 'lit-html/directives/style-map';
import "@elements/core/buttons/icon";
import "./base";

@customElement("tooltip-error")
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

    onGlobalMouseDown = (evt: MouseEvent) => {
        if(!evt.composedPath().includes(this.shadowRoot?.getElementById("tooltip") as any)) {
            this.dispatchEvent(new Event("close"))
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

    render() {
        const {target, maxWidth, placement, offsetSkidding, offsetDistance} = this;

        let bodyStyles:any = {
        };

        if(maxWidth !== -1) {
            bodyStyles.maxWidth = `${maxWidth}px`;
        }
        return html`

            <tooltip-base id="tooltip" color="red" .target=${target} .placement=${placement} .offsetSkidding=${offsetSkidding} .offsetDistance=${offsetDistance}>
                <article>
                    <img-ui path="core/tooltips/alert.svg"></img-ui>
                    <div class="body" style="${styleMap(bodyStyles)}"><slot></slot></div>
                </article>
            </tooltip-base>

        `;
    }
}
