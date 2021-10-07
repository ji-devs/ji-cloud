import {
    LitElement,
    html,
    css,
    customElement,
    property,
    query,
} from "lit-element";
import { queryPierceShadow } from "@utils/dom";
import "./container";
import "./content";
import {MoveStrategy, ZLayer, Anchor, ContentAnchor, TrackerProp} from "./content";
import "@elements/core/drag/container";

@customElement("overlay-drag")
export class _ extends LitElement {
    static get styles() {
        return [css``];
    }

    @property({ type: Boolean, reflect: true })
    disableChildPointer: boolean = false;

    //pass through
    @property()
    container:TrackerProp | undefined = "mainOrWindow";

    @property()
    target:TrackerProp | undefined;

    @property()
    strategy:MoveStrategy = "none";

    @property({reflect: true})
    zLayer:ZLayer | undefined = "drag";

    @property()
    contentAnchor:ContentAnchor = "oppositeH";

    @property()
    targetAnchor:Anchor = "tr";

    @property({type: Number})
    marginX:number = 0;

    @property({type: Number})
    marginY:number = 0;

    render() {

        const {container, target, strategy, zLayer,marginX, marginY, contentAnchor, targetAnchor, } = this;
        // return html`<slot></slot>`
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
            >
                <drag-container
                    .disableChildPointer=${this.disableChildPointer}
                    @close=${() => this.dispatchEvent(new Event("close"))}
                >
                    <slot></slot>
                </drag-container>
            </overlay-content>
        `;
    }
}
