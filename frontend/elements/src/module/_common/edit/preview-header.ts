import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import { ModuleKind, STR_MODULE_PREVIEW_TOOLTIP_BODY } from "@elements/module/_common/types";

const STR_TITLE = "Preview Mode";

const STR_TOOLTIP_CONTINUE = "Click to continue";
const STR_TOOLTIP_GETTING_STARTED = "Time to play!";

@customElement("module-preview-header")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                section {
                    display: grid;
                    grid-template-columns: auto auto;
                    width: 100%;
                    height: 120px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: var(--white);
                    justify-content: space-between;
                    align-items: center;
                }
                @media (min-width: 1920px) {
                    section {
                        height: 150px;
                    }
                }
                .nav {
                    width: 392px;
                    box-sizing: border-box;
                    height: 100%;
                    padding-top: 24px;
                    padding-left: 20px;
                    padding-right: 20px;
                    grid-row: 1;
                    grid-column: 1;
                    z-index: 1;
                }
                @media (min-width: 1920px) {
                    .nav {
                        width: 556px;
                        padding-top: 40px;
                        padding-left: 50px;
                        padding-right: 50px;
                    }
                }
                .btn {
                    grid-row: 1;
                    grid-column: 2;
                    z-index: 1;
                }
                .title {
                    grid-column: 1 / -1;
                    grid-row: 1;
                    text-align: center;
                    font-size: 28px;
                    color: var(--dark-blue-4);
                }
            `,
        ];
    }

    firstUpdated(_changed: any) {
        this.sectionRef = this.shadowRoot?.getElementById(
            "section"
        ) as HTMLElement;
        this.requestUpdate();
    }

    @property()
    moduleKind: ModuleKind = "memory";

    @property({ type: Boolean })
    continueTooltip: boolean = false;

    sectionRef: HTMLElement | undefined;

    render() {
        const { sectionRef, moduleKind, continueTooltip } = this;
        return html`
            <section id="section">
                <div class="nav">
                    <slot name="nav"></slot>
                </div>
                <div class="title">${STR_TITLE}</div>
                <div class="btn"><slot name="btn"></slot></div>
            </section>
            ${sectionRef ? renderIntroTooltip(moduleKind, sectionRef) : nothing}
            ${sectionRef && continueTooltip
                ? renderContinueTooltip(sectionRef)
                : nothing}
        `;
    }
}

function renderIntroTooltip(moduleKind: ModuleKind, targetRef: HTMLElement) {
    const body = STR_MODULE_PREVIEW_TOOLTIP_BODY[moduleKind];
    if (!body) {
        return nothing;
    }

    const showId = `preview-header-intro-${moduleKind}`;

    return html`
        <overlay-container>
            <overlay-tooltip-info
                id="tooltip"
                .target=${targetRef}
                targetAnchor="bl"
                contentAnchor="tl"
                title="${STR_TOOLTIP_GETTING_STARTED}"
                body="${body}"
                showId="${showId}"
                closeable
            >
            </overlay-tooltip-info>
        </overlay-container>
    `;
}
function renderContinueTooltip(targetRef: HTMLElement) {
    return html`
        <overlay-container>
            <overlay-tooltip-info
                id="tooltip"
                .target=${targetRef}
                targetAnchor="br"
                contentAnchor="tr"
                title=""
                body="${STR_TOOLTIP_CONTINUE}"
                closeable
            >
            </overlay-tooltip-info>
        </overlay-container>
    `;
}
