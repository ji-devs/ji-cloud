import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from 'lit-html/directives/class-map';
import "@elements/core/images/ui";

export type IconKind = "circle-x-blue" | "circle-check" | "circle-kebab-grey" | "circle-kebab-blue" | "circle-pencil" | "gears" | "x";

@customElement("button-icon")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        :host {
          cursor: pointer;
        }
        .box32 {
            display: flex;
            justify-content: center;
            align-items: center;
            width: 32px;
            height: 32px;
        }
            img-ui {
                width: inherit;
                height: inherit;
                object-fit: inherit;
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
        : icon === "circle-x-blue" ? "circle-x-blue.svg"
        : "";

    const path = `core/buttons/icon/${filename}`;

    const classes = classMap({
        ["box32"]: icon === "x"
    });

    return icon === "x" ? html`<div class="${classes}"><img-ui path="${path}"></img-ui></div>`
        : html`<img-ui path="${path}"></img-ui>`
  }
}
