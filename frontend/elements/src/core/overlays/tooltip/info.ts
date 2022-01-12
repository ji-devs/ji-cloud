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
import { Color } from "./container";

const STR_NO_SHOW_AGAIN = "Don’t show tips again";

@customElement("overlay-tooltip-info")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                }

                overlay-content {
                    animation: jump 1s ease-in-out;
                }
                @keyframes jump {
                    0% {
                        transform: translateY(0px);
                    }
                    20% {
                        transform: translateY(8px);
                    }
                    40% {
                        transform: translateY(-6px);
                    }
                    60% {
                        transform: translateY(4px);
                    }
                    80% {
                        transform: translateY(-2px);
                    }
                    100% {
                        transform: translateY(0px);
                    }
                }

                .content {
                    display: flex;
                    flex-direction: column;
                }

                .close-button {
                    align-self: flex-end;
                    background-color: transparent;
                    position: absolute;
                    margin-right: -16px;
                    font-size: 20px;
                    border: 0;
                    padding: 0;
                    height: 24px;
                    width: 24px;
                    cursor: pointer;
                    color: var(--light-blue-4);
                }
                .title {
                    margin-top: 20px;
                    font-size: 18px;
                    font-weight: 900;
                    color: #ffffff;
                }
                .body {
                    font-size: 14px;
                    letter-spacing: -0.18px;
                    color: #ffffff;
                    width: 304px;
                    margin: 8px 0 36px 0;
                }
                .noshow {
                    font-size: 13px;
                    font-weight: 500;
                    color: var(--light-blue-4);
                    cursor: pointer;
                    align-self: flex-end;
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

    onClose = () => {
        this.dispatchEvent(new Event("close"));
        this.selfClosed = true;
    };

    @property()
    title: string = "";

    @property()
    body: string = "";

    @property({ type: Boolean })
    showPermanentlyClose: boolean = false;

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

    @property()
    color: Color = "blue";

    @property({ type: Number })
    arrowNudge: number = 0;


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

    renderPermanentlyClose() {
        if (!this.showPermanentlyClose) {
            return nothing;
        }

        const onClick = () => {
            this.dispatchEvent(new Event("permanently-close"));
            this.onClose();
        };
        return html`
            <div @click=${onClick} class="noshow">${STR_NO_SHOW_AGAIN}</div>
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
            title,
            body,
            arrowNudge,
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
                <tooltip-container
                    id="tooltip"
                    .color=${this.color}
                    .contentAnchor=${this.currContentAnchor}
                    .targetAnchor=${this.currTargetAnchor}
                    .arrowNudge=${arrowNudge}
                >
                    <section class="content">
                        ${this.renderClose()}
                        ${title !== ""
                            ? html`<div class="title">${title}</div>`
                            : nothing}
                        ${body !== ""
                            ? html`<section class="body">${body}</section>`
                            : nothing}
                        ${this.renderPermanentlyClose()}
                    </section>
                </tooltip-container>
            </overlay-content>
        `;
    }
}

