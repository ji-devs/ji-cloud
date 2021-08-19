import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import "@elements/core/buttons/icon";
import "./base";
import {COLOR, Placement, ElementTarget, MoveStrategy} from "./base";

const STR_NO_SHOW_AGAIN = "Don't show again";

@customElement("tooltip-info")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
            }

            .content {
                display: flex;
                flex-direction: column;
            }

            .close {
                align-self: flex-end;
            }
            .title {
              font-size: 28px;
              font-weight: 900;
              color: var(--dark-blue-4);
            }
            .body {
              font-size: 18px;
              font-weight: 300;
              letter-spacing: -0.18px;
              color: #383838;
              width: 304px;
            }
            .noshow {
              font-size: 16px;
              font-weight: 500;
              color: var(--main-blue);
              cursor: pointer;
            }
            `
        ];
    }

    onClose = () => {
        this.closed = true;
        this.dispatchEvent(new Event("close"));
    }

    @property({type: Boolean})
    closed:boolean = false;


    @property()
    title:string = "";

    @property()
    body:string = "";

    @property()
    showId:string | "debug" = "";

    @property({type: Boolean})
    closeable:boolean = false;


    //pass through
    @property({attribute: false})
    container:Element | Window = window;

    @property()
    moveStrategy:MoveStrategy = "";

    @property({reflect: true})
    color:COLOR = "beige";

    @property()
    target:ElementTarget | undefined;

    @property()
    placement:Placement = "bottom";

    @property({type: Number})
    margin:number = 0;

    @property({type: Number})
    arrowOffset:number = 0;

    render() {
        const {container, moveStrategy, closed, title, body, showId, target, placement, closeable, color, margin, arrowOffset} = this;

        if(closed) {
            return nothing;
        }

        if(showId !== "" && showId !== "debug") {
            if(sessionStorage.getItem(showId) === "hidden") {
                //hiding due to storage
                return nothing;
            }
        }

        return html`
            <tooltip-base id="tooltip" color=${color} .closed=${closed} .container=${container} .moveStrategy=${moveStrategy} .target=${target} .placement=${placement} margin=${margin} arrowOffset=${arrowOffset}>
                <section class="content">
                    ${closeable ? renderClose(this.onClose) : nothing}
                    ${title !== "" ? html`<div class="title">${title}</div>` : nothing}
                    ${body !== "" ? html`<section class="body">${body}</section>` : nothing}
                    ${showId !== "" ? renderShowId(showId, this.onClose) : nothing}       
                </section>
            </tooltip-base>

        `;
    }
}

function renderClose(onClose: () => any) {
    return html`<button-icon class="close" icon="circle-x-blue" @click=${onClose}></button-icon>`
}

function renderShowId(showId:string, onClose: () => any) {
    const onClick = () => {
        if(showId === "debug") {
            //skipping showId action because it's debug
        } else {
            //setting ${showId}
            sessionStorage.setItem(showId, "hidden");
        }

        onClose();
    }
    return html`<div @click=${onClick} class="noshow">${STR_NO_SHOW_AGAIN}</div>`;
}
