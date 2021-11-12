import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import { styleMap } from "lit-html/directives/style-map";
import "@elements/core/overlays/container";
import "@elements/core/overlays/content";
import {
    TrackerProp,
    ZLayer,
    Anchor,
    ContentAnchor,
    MoveStrategy,
} from "@elements/core/overlays/content";
import "@elements/core/buttons/icon";
import "./container";
import { Color } from "./container";

@customElement("overlay-tooltip-error")
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
            `,
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
        this.dispatchEvent(new Event("accept"));
    };
    onCancel = () => {
        this.dispatchEvent(new Event("close"));
    };
    onGlobalMouseDown = (evt: MouseEvent) => {
        if (
            !evt
                .composedPath()
                .includes(this.shadowRoot?.getElementById("tooltip") as any)
        ) {
            this.onCancel();
        }
    };

    @property({ type: Number })
    maxWidth: number = -1;

    //internal
    @property()
    currContentAnchor: ContentAnchor = "oppositeH";

    @property()
    currTargetAnchor: Anchor = "tr";

    //pass through
    @property()
    container: TrackerProp | undefined = window;

    @property()
    target: TrackerProp | undefined;

    @property()
    strategy: MoveStrategy = "";

    @property({ reflect: true })
    zLayer: ZLayer | undefined = "tooltip";

    @property()
    contentAnchor: ContentAnchor = "oppositeH";

    @property()
    targetAnchor: Anchor = "tr";

    @property({ type: Number })
    marginX: number = 0;

    @property({ type: Number })
    marginY: number = 0;

    @property()
    color: Color = "red";

    @property({ type: Number })
    arrowNudge: number = 0;

    render() {
        const {
            container,
            target,
            strategy,
            zLayer,
            marginX,
            marginY,
            contentAnchor,
            targetAnchor,
            maxWidth,
            arrowNudge,
        } = this;

        const bodyStyles: any = {};

        if (maxWidth !== -1) {
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
                @anchor-changed=${(evt: CustomEvent) => {
                    const { contentAnchor, targetAnchor } = evt.detail;
                    this.currContentAnchor = contentAnchor;
                    this.currTargetAnchor = targetAnchor;
                }}
            >
                <tooltip-container
                    id="tooltip"
                    color="red"
                    .color=${this.color}
                    .contentAnchor=${this.currContentAnchor}
                    .targetAnchor=${this.currTargetAnchor}
                    .arrowNudge=${arrowNudge}
                >
                    <article>
                        <img-ui path="core/tooltips/alert.svg"></img-ui>
                        <div class="body" style="${styleMap(bodyStyles)}">
                            <slot></slot>
                        </div>
                    </article>
                </tooltip-container>
            </overlay-content>
        `;
    }
}
