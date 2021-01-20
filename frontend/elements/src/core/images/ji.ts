import { LitElement, html, css, customElement, property } from "lit-element";
import { imageLib } from "@utils/path";
import "./basic";

@customElement("img-ji")
export class _ extends LitElement {
  @property()
  lib: "global" | "user" | "web" = "global";

  @property()
  size: "original" | "full" | "thumb" = "full";

  @property()
  id: string = "";

  render() {
    const { lib, size, id } = this;

    const src = imageLib({ lib, size, id });

    return html`<img-basic id="img" src="${src}"></img>`;
  }
}
