import { LitElement, html, css, customElement, property } from "lit-element";
import { mediaUi} from "@utils/path";
import {sameOrigin} from "@utils/image";

@customElement("img-ui")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            img {
                all: inherit;
            }
            `,
        ];
    }

      @property()
      path: string = "";

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
    const { path } = this;

    const src = mediaUi(path);

        if (sameOrigin(src)) {
            return html`<img .src="${src}" @load="${this.onLoad}" ></img>`;
        } else {
            return html`<img .src="${src}" crossorigin="anonymous" @load="${this.onLoad}" ></img>`;
            }
    }
}
