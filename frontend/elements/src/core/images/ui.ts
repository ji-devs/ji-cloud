import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { mediaUi } from "@utils/path";
import "./basic";

@customElement("img-ui")
export class _ extends LitElement {
  @property()
  path: string = "";

  render() {
    const { path } = this;

    const src = mediaUi(path);

    return html`<img-basic src="${src}"></img>`;
  }
}
