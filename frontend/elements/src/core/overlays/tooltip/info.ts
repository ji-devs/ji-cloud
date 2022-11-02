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
import { faSortSizeDown } from "@fortawesome/pro-solid-svg-icons";

type Size = "regular" | "large";

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
                    font-family: Poppins, Alef;
                    font-size: 14px;
                    letter-spacing: -0.18px;
                    color: #ffffff;
                    width: 304px;
                    margin: 8px 0 36px 0;
                    white-space: pre-wrap;
                    overflow-wrap: break-word;
                }
                :host([size="large"]) .body {
                    font-size: 22px;
                    width: max-content;
                    max-width: 504px;
                    min-width: 304px;
                }

                :host([color="dark-blue"]) .body {
                    color: #f78c83;
                }

                :host([removeMargins]) .body {
                    margin: inherit;
                }
                :host([color="light-orange"]) .body {
                    color: var(--dark-gray-6);
                }

                :host([centeredContent]) .body {
                    text-align: center;
                }

                .actions {
                    display: flex;
                    flex-direction: row;
                    justify-content: space-between;
                }
                .actions :last-child {
                    margin-left: auto;
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

    @property({ reflect: true })
    color: Color = "blue";

    @property({ reflect: true })
    size: Size = "regular";

    @property({ type: Number })
    arrowNudge: number = 0;

    @property({ type: Boolean, reflect: true })
    removeMargins: boolean = false;

    @property({ type: Boolean, reflect: true })
    centeredContent: boolean = false;

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
                            ? html`<section dir="auto" class="body">${body}</section>`
                            : html`<section class="body"><slot name="body"></slot></section>`}
                        <div class="actions">
                            <slot name="actions"></slot>
                        </div>
                    </section>
                </tooltip-container>
            </overlay-content>
        `;
    }
}

