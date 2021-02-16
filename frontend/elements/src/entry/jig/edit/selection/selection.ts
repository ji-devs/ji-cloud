import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import "@elements/entry/jig/_common/bg";

@customElement("jig-edit-selection")
export class _ extends LitElement {
  static get styles() {
    return [
        css`
            `,
    ];
  }


  render() {
      return html`
              Hello world
    `;
  }
}
