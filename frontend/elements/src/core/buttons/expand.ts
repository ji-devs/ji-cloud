import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

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

  onToggle() {
    this.expanded = !this.expanded;

    this.dispatchEvent(new CustomEvent("custom-toggle", {
      detail: { value: this.expanded},
    }))
  }

  @property({type: Boolean})
  expanded: boolean = true;

  render() {
    const { expanded } = this;

    const icon = expanded ? "Icon_ExpandAll_24.svg": "Icon_CollapseAll_24.svg";

    return html`<img-ui path="${icon}" @click="${this.onToggle.bind(this)}"></img-ui>`;
  }
}
