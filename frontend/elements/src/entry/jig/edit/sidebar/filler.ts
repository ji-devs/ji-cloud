import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import "@elements/core/buttons/icon";
import {ModuleKind, GET_STR_MODULE} from "@elements/entry/jig/module-types";

@customElement("jig-edit-sidebar-filler")
export class _ extends LitElement {
  static get styles() {
    return [
        css`
            section {
              width: 416px;
              height: 168px;
              background-color: #e7f0fd;
              z-index: 1;
              position: relative;
              top: 0;
              left: 0;
            }
            `,
    ];
  }
  render() {
      return html`<section></section>`
  }
}
