import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import "@elements/core/inputs/text-pencil";
import "@elements/core/buttons/icon";
import "@elements/core/buttons/text";


@customElement("jig-edit-sidebar-header")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
      :host {
          display: grid;
          grid-template-columns: 1fr 1fr;
          grid-template-rows: 32px 40px 1fr;
          gap: 0px 0px;
          grid-template-areas:
            ". close"
            "logo gallery"
            "input input";
      }

      .close {
        grid-area: close;
        justify-self: end;
      }

      .logo {
        grid-area: logo;
      }

      .gallery {
        grid-area: gallery;
        justify-self: end;
        align-self: end;
      }

      .input {
        margin-top: 23px;
        width: 100%;
        grid-area: input;
      }
      `,
    ];
  }

  render() {

    return html`
        <div class="close"><slot name="close"></slot></div>
        <img-ui class="logo" path="entry/jig/logo-jigzi.svg"></img-ui>
        <div class="gallery"><slot name="gallery"></slot></div>
        <div class="input"><slot name="input"></slot></div>
    `;
  }
}
