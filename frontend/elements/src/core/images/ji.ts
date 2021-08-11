import { LitElement, html, css, customElement, property } from "lit-element";
import { imageLib, MediaLibOptions, MediaSizeOptions } from "@utils/path";
import {sameOrigin} from "@utils/image";
import {nothing} from "lit-html";

@customElement("img-ji")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                img {
                    display: inherit;
                    width: inherit;
                    height: inherit;
                    max-height: 100%;
                    max-width: 100%;
                    object-fit: inherit;
                }
            `,
        ];
    }

    @property({ type: Boolean })
    fallbackVisible: boolean = false;

    @property({ type: Boolean })
    draggable: boolean = true;

    @property({ type: Boolean })
    cacheBust: boolean = false;

    @property()
    lib: MediaLibOptions = "global";

    @property()
    size: MediaSizeOptions = "full";

    //use with cacheBust true to force reloading when id changes to the same thing
    @property({ hasChanged: () => true })
    id: string = "";

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

    onError(evt: Event) {
        this.fallbackVisible = true;
    }

    render() {
        const { lib, size, id, fallbackVisible, cacheBust, draggable } = this;

        let src = imageLib({ lib, size, id });

        if (cacheBust) {
            src += `?cb=${Date.now()}`;
        }

        if (fallbackVisible) {
            return html`<slot name="fallback"><div>[MISSING IMAGE]</div></slot>`;
        } else {
            if (sameOrigin(src)) {
                return html`<img .draggable=${draggable} .src="${src}" @error=${this.onError} @load="${this.onLoad}" ></img>`;
            } else {
                return html`<img .draggable=${draggable} .src="${src}" crossorigin="anonymous" @error=${this.onError} @load="${this.onLoad}" ></img>`;
            }
        }
    }
}
