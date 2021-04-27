import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { styleMap } from 'lit-html/directives/style-map';
import "@elements/core/buttons/icon";
import "./base";
import {COLOR, Placement, ElementTarget, MoveStrategy} from "./base";

@customElement("tooltip-error")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            :host {
              font-family: Poppins;
              box-shadow: 0 3px 40px 0 rgba(0, 0, 0, 0.08);
                    display: inline-block;
            }

                :host([color="beige"]) {
                    border: solid 2px var(--light-orange-2);
                    background-color: var(--light-orange-1);
                }
                :host([color="red"]) {
                    background-color: var(--light-red-1);
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

    //pass through
    @property()
    container:Element | Window = window;

    @property()
    moveStrategy:MoveStrategy = "";

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
        const {container, moveStrategy, target, maxWidth, placement, color, margin, arrowOffset} = this;

        let bodyStyles:any = {
        };

        if(maxWidth !== -1) {
            bodyStyles.maxWidth = `${maxWidth}px`;
        }
        return html`

            <tooltip-base id="tooltip" color=${color} .container=${container} .moveStrategy=${moveStrategy} .target=${target} .placement=${placement} margin=${margin} arrowOffset=${arrowOffset}>
                <article>
                    <img-ui path="core/tooltips/alert.svg"></img-ui>
                    <div class="body" style="${styleMap(bodyStyles)}"><slot></slot></div>
                </article>
            </tooltip-base>

        `;
    }
}
