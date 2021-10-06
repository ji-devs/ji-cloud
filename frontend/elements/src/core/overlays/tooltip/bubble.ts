import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { styleMap } from 'lit-html/directives/style-map';
import "@elements/core/overlays/container";
import "@elements/core/overlays/content";
import {TrackerProp, ZLayer, Anchor, ContentAnchor, MoveStrategy} from "@elements/core/overlays/content";
import "@elements/core/buttons/icon";
import "./container";
import {Color} from "./container";

@customElement("overlay-tooltip-bubble")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                }

            .body {
                font-size: 30rem;
                font-weight: 600;
              text-align: center;
              color: var(--white);
            }
            article {
                display: flex;
                gap: 16rem;
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
    color:Color = "green";

    @property()
    targetAnchor:Anchor = "tr";

    @property({type: Number})
    marginX:number = 0;

    @property({type: Number})
    marginY:number = 0;

    
    @property({type: Number})
    arrowNudge:number = 0;

    render() {
        const {container, target, strategy, zLayer,marginX, marginY, contentAnchor, targetAnchor, maxWidth, arrowNudge} = this;

        let bodyStyles:any = {
        };

        if(maxWidth !== -1) {
            bodyStyles.maxWidth = `${maxWidth}px`;
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
                <article>
                    <div class="body" style="${styleMap(bodyStyles)}"><slot></slot></div>
                </article>
                </tooltip-container>
            </overlay-content>

        `;
    }
}
