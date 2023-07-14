import { LitElement, html, css, customElement, property } from "lit-element";
import { imageLib, MediaLibOptions, MediaSizeOptions } from "@utils/path";
import { sameOrigin } from "@utils/image";
import { nothing } from "lit-html";

@customElement("img-ji")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: relative;
                }
                img {
                    display: inherit;
                    width: inherit;
                    height: inherit;
                    max-height: 100%;
                    max-width: 100%;
                    object-fit: inherit;
                    border-radius: var(--border-radius, 0);
                }
                .premium {
                    padding: 0 4px;
                    border-radius: 2px;
                    border: solid 1px var(--light-blue-3);
                    background-color: var(--white);
                    position: absolute;
                    left: 6px;
                    bottom: 6px;
                    height: 12px;
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                }
                .premium img-ui {
                    width: 11px;
                }
                .premium .premium-label {
                    display: none;
                    font-family: 'Poppins', sans-serif;
                    font-size: 11px;
                    font-weight: 600;
                    margin-left: 6px;
                }
                .premium:hover .premium-label {
                    display: inline-block;
                }
            `,
        ];
    }

    @property({ type: Boolean })
    fallbackVisible: boolean = false;

    @property({ type: Boolean })
    draggable: boolean = true;

    @property()
    cacheBust: any = false;

    @property()
    lib: MediaLibOptions = "global";

    @property()
    size: MediaSizeOptions = "full";

    //use with cacheBust true to force reloading when id changes to the same thing
    @property({ hasChanged: () => true })
    id: string = "";

    //use with cacheBust true to force reloading when id changes to the same thing
    @property()
    borderRadius: string = "0";

    @property({ type: Boolean })
    premium: boolean = false;

    firstUpdated() {
        this.style.setProperty('--border-radius', this.borderRadius);
    }

    updated(changedProperties: Map<string | number | symbol, unknown>) {
        if (changedProperties.has('borderRadius')) {
            this.style.setProperty('--border-radius', this.borderRadius);
        }
    }


    onLoad(evt: Event) {
        const img = evt.currentTarget as HTMLImageElement;
        const width = img.naturalWidth;
        const height = img.naturalHeight;

        this.dispatchEvent(
            new CustomEvent("image-load", {
                detail: { width, height },
                bubbles: true,
                composed: true,
            })
        );
    }

    onError(_evt: Event) {
        this.dispatchEvent(
            new Event("image-error", {
                bubbles: true,
                composed: true,
            })
        );
        this.fallbackVisible = true;
    }

    render_image() {
        const { lib, size, id, fallbackVisible, cacheBust, draggable } = this;

        let src = imageLib({ lib, size, id });

        if (cacheBust) {
            src += `?cb=${Date.now()}`;
        }

        if (fallbackVisible) {
            return html`<slot name="fallback"
                ><div>[MISSING IMAGE]</div></slot
            >`;
        } else {
            if (sameOrigin(src)) {
                return html`<img
                    .draggable=${draggable}
                    .src="${src}"
                    @error=${this.onError}
                    @load="${this.onLoad}"
                />`;
            } else {
                return html`<img
                    .draggable=${draggable}
                    .src="${src}"
                    crossorigin="anonymous"
                    @error=${this.onError}
                    @load="${this.onLoad}"
                />`;
            }
        }
    }

    render() {
        let image_html = this.render_image();

        return html`
            ${this.premium
                ? html`<span class="premium"><img-ui
                    class="jiggling"
                    path="icons/pro-icon-small.svg"
                ></img-ui><span class="premium-label">Pro</span></span>`
                : nothing
            }
            ${image_html}
        `;
    }
}
