import { LitElement, html, css, customElement, property } from "lit-element";
import { mediaUi} from "@utils/path";
import {sameOrigin} from "@utils/image";

@customElement("img-ui")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            img {
                display: inherit;
                width: inherit;
                height: inherit;
                object-fit: inherit;
                object-position: inherit;
            }

            `,
        ];
    }

      @property()
      path: string = "";

      @property({type: Boolean})
      draggable: boolean = true; 

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


    render() {
    const { path, draggable } = this;

    const src = mediaUi(path);

        if (sameOrigin(src)) {
            return html`<img .draggable=${draggable} .src="${src}" @load="${this.onLoad}" ></img>`;
        } else {
            return html`<img .draggable=${draggable} .src="${src}" crossorigin="anonymous" @load="${this.onLoad}" ></img>`;
            }
    }
}
