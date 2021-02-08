import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

export type Mode = "expanded" | "collapsed";

@customElement("button-expand")
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
  mode: Mode = "expanded";

  render() {
    const { mode } = this;

    const icon =
      mode === "collapsed" ? "Icon_CollapseAll_24.svg" : "Icon_ExpandAll_24.svg";

    return html`<img-ui path="${icon}"></img-ui>`;
  }
}
