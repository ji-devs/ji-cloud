import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import { classMap } from "lit-html/directives/class-map";
import { mediaUi } from "@utils/path";

@customElement("img-basic")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        :host {
          display: inline-block;
        }

        img {
          width: 100%;
          height: 100%;
        }
      `,
    ];
  }

  @property()
  src: string = "";

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
    const { src } = this;
    if (sameOrigin(src)) {
      return html`<img src="${src}" @load="${this.onLoad}" ></img>`;
    } else {
      return html`<img src="${src}" crossorigin="anonymous" @load="${this.onLoad}" ></img>`;
    }
  }
}

function sameOrigin(url: string): boolean {
  const is_web = url.indexOf("http://") === 0 || url.indexOf("https://") === 0;

  if (is_web) {
    const locationOrigin = window.location.origin;
    const urlOrigin = new URL(url).origin;
    return urlOrigin == locationOrigin;
  } else {
    return true;
  }
}
