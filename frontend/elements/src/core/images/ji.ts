import { LitElement, html, css, customElement, property } from "lit-element";
import { imageLib } from "@utils/path";
import {sameOrigin} from "@utils/image";

@customElement("img-ji")
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

    @property({type: Boolean})
    cacheBust:boolean = false;

    @property()
    lib: "global" | "user" | "web" = "global";

    @property()
    size: "original" | "full" | "thumb" = "full";

    //use with cacheBust true to force reloading when id changes to the same thing
    @property({hasChanged: () => true})
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


    render() {
        const { lib, size, id, cacheBust } = this;

        let src = imageLib({ lib, size, id });

        if(cacheBust) {
            src += `?cb=${Date.now()}`;
        }

        if (sameOrigin(src)) {
            return html`<img .src="${src}" @load="${this.onLoad}" ></img>`;
        } else {
            return html`<img .src="${src}" crossorigin="anonymous" @load="${this.onLoad}" ></img>`;
            }
    }
}
