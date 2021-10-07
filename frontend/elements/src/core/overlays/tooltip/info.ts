import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { styleMap } from 'lit-html/directives/style-map';
import "@elements/core/overlays/container";
import "@elements/core/overlays/content";
import {TrackerProp, ZLayer, Anchor, ContentAnchor, MoveStrategy} from "@elements/core/overlays/content";
import "@elements/core/buttons/icon";
import "./container";
import {Color} from "./container";

const STR_NO_SHOW_AGAIN = "Don't show again";

@customElement("overlay-tooltip-info")
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

    onClose = () => {
        this.dispatchEvent(new Event("close"));
        this.selfClosed = true;
    }


    @property()
    title:string = "";

    @property()
    body:string = "";

    @property()
    showId:string | "debug" = "";

    @property({type: Boolean})
    closeable:boolean = false;

    @property({type: Boolean})
    selfClosed:boolean = false;

    //internal
    @property()
    currContentAnchor:ContentAnchor = "oppositeH";

    @property()
    currTargetAnchor:Anchor = "tr";

    //pass through
    @property()
    container:TrackerProp | undefined = window;

    @property()
    target:TrackerProp | undefined;

    @property()
    strategy:MoveStrategy = "";

    @property({reflect: true})
    zLayer:ZLayer | undefined = "tooltip";

    @property()
    contentAnchor:ContentAnchor = "oppositeH";

    @property()
    targetAnchor:Anchor = "tr";

    @property({type: Number})
    marginX:number = 0;

    @property({type: Number})
    marginY:number = 0;

    @property()
    color:Color = "beige";
    
    @property({type: Number})
    arrowNudge:number = 0;

    render() {
        const {container, selfClosed, target, strategy, zLayer,marginX, marginY, contentAnchor, targetAnchor, closeable, title, body, showId, arrowNudge} = this;

        if(selfClosed) {
            return(nothing);
        }

        if(showId !== "" && showId !== "debug") {
            if(sessionStorage.getItem("tooltip-" + showId) === "hidden") {
                //hiding due to storage
                return nothing;
            }
        }

        return html`

            <overlay-content
             .container=${container}
             .target=${target}
             .strategy=${strategy}
             .zLayer=${zLayer}
             .contentAnchor=${contentAnchor}
             .targetAnchor=${targetAnchor}
             .marginX=${marginX}
             .marginY=${marginY}
             @anchor-changed=${(evt:CustomEvent) => {
                const {contentAnchor, targetAnchor} = evt.detail;
                 this.currContentAnchor = contentAnchor;
                 this.currTargetAnchor = targetAnchor;
             }}
            >
                <tooltip-container
                    id="tooltip"
                    .color=${this.color}
                    .contentAnchor=${this.currContentAnchor}
                    .targetAnchor=${this.currTargetAnchor}
                    .arrowNudge=${arrowNudge}
                >
                <section class="content">
                    ${closeable ? renderClose(this.onClose) : nothing}
                    ${title !== "" ? html`<div class="title">${title}</div>` : nothing}
                    ${body !== "" ? html`<section class="body">${body}</section>` : nothing}
                    ${showId !== "" ? renderShowId(showId, this.onClose) : nothing}       
                </section>
                </tooltip-container>
            </overlay-content>

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
            sessionStorage.setItem("tooltip-" + showId, "hidden");
        }

        onClose();
    }
    return html`<div @click=${onClick} class="noshow">${STR_NO_SHOW_AGAIN}</div>`;
}
