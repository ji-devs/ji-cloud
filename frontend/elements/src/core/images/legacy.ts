import { LitElement, html, css, customElement, property } from "lit-element";
import { legacyMock, legacyMedia } from "@utils/path";
import {sameOrigin} from "@utils/image";

@customElement("img-legacy")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            img {
                width: inherit;
                height: inherit;
                object-fit: inherit;
            }
            `,
        ];
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

      @property()
      jigId: string = "";

      @property()
      moduleId: string = "";

      @property()
      path: string = "";

      @property({ type: Boolean })
      mock: boolean = false;

    render() {
    const { jigId, moduleId, path, mock } = this;

    const src = mock
      ? legacyMock({ jigId, moduleId, path })
      : legacyMedia({ jigId, moduleId, path });

        if (sameOrigin(src)) {
            return html`<img .src="${src}" @load="${this.onLoad}" ></img>`;
        } else {
            return html`<img .src="${src}" crossorigin="anonymous" @load="${this.onLoad}" ></img>`;
            }
    }
}
