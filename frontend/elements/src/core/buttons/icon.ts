import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

export type IconKind = "circle-check" | "circle-kebab-grey" | "circle-kebab-blue" | "circle-pencil" | "gears" | "x";

@customElement("button-icon")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        :host {
          cursor: pointer;
        }
      `,
    ];
  }

  @property()
  icon: IconKind = "circle-check";

  render() {
    const { icon } = this;

    const filename = icon === "circle-check" ? "circle-check-green.svg"
        : icon === "circle-kebab-grey" ? "circle-kebab-grey.svg"
        : icon === "circle-kebab-blue" ? "circle-kebab-blue.svg"
        : icon === "circle-pencil" ? "circle-pencil-blue.svg"
        : icon === "gears" ? "gears-plus-blue.svg"
        : icon === "x" ? "x.svg"
        : "";

    const path = `core/buttons/icon/${filename}`;


    return html`<img-ui path="${path}"></img-ui>`;
  }
}
