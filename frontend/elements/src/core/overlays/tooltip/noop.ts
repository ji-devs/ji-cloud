import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import {
    TrackerProp,
    ZLayer,
    Anchor,
    ContentAnchor,
    MoveStrategy,
} from "@elements/core/overlays/content";
import "@elements/core/buttons/icon";
import "./container";

@customElement("overlay-tooltip-noop")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
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

    onClose = () => {
        this.dispatchEvent(new Event("close"));
        this.selfClosed = true;
    };

    @property({ type: Boolean })
    closeable: boolean = false;

    @property({ type: Boolean })
    selfClosed: boolean = false;

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

    renderClose() {
        if (!this.closeable) {
            return nothing;
        }

        return html`
            <button class="close-button" @click=${this.onClose}>
                <fa-icon icon="fa-light fa-xmark"></fa-icon>
            </button>
        `;
    }

    render() {
        const {
            container,
            selfClosed,
            target,
            strategy,
            zLayer,
            marginX,
            marginY,
            contentAnchor,
            targetAnchor,
        } = this;

        if (selfClosed) {
            return nothing;
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
            </overlay-content>
        `;
    }
}

