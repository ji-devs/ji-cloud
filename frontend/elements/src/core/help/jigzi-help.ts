import {
    LitElement,
    html,
    css,
    customElement,
    property,
    state,
    query,
} from "lit-element";
import { nothing } from "lit-html";

const INFO_TOOLTIP_DELAY = 1_500;

@customElement("jigzi-help")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                #gear-img {
                    cursor: pointer;
                }
            `,
        ];
    }

    @property()
    title: string = "";

    @property()
    body: string = "";

    @property()
    showId: string = "";

    @query("#gear-img")
    imgRef: HTMLElement | undefined;

    @state()
    showInfoTooltip: boolean = false;

    //instead of firstUpdated since tooltip needs the size of the image to position correctly
    onImageLoaded() {
        this.requestUpdate();
    }

    onGearClick() {
        const tooltipRef = this.shadowRoot?.getElementById("tooltip");

        (tooltipRef as any).selfClosed = false;
    }

    connectedCallback() {
        super.connectedCallback();

        this.showInfoTooltipDelayed();
    }

    private showInfoTooltipDelayed() {
        setTimeout(() => {
            this.showInfoTooltip = true;
        }, INFO_TOOLTIP_DELAY);
    }

    render() {
        const marginX = -33;

        return html`
            <img-ui
                @click=${this.onGearClick}
                @image-load=${this.onImageLoaded}
                id="gear-img"
                path="module/_common/edit/header/jiggling-gear.png"
            ></img-ui>
            <overlay-container>
                ${this.showInfoTooltip
                    ? html`
                          <overlay-tooltip-info
                              id="tooltip"
                              .target=${this.imgRef}
                              .marginX=${marginX}
                              targetAnchor="bm"
                              contentAnchor="tr"
                              title=${this.title}
                              body=${this.body}
                              showId=${this.showId}
                              closeable
                              strategy="track"
                          ></overlay-tooltip-info>
                      `
                    : nothing}
            </overlay-container>
        `;
    }
}
