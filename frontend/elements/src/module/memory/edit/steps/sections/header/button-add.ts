import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import "@elements/module/_common/header";
import {ButtonFlex} from "@elements/generic/buttons/flex";

export type MODE = "pair";
const STR_PAIR = "Add a pair";

@customElement('header-button-add')
export class _ extends ButtonFlex {
  static get styles() {
      return [...super.styles, css`
          :host {
              margin: 16px 0; 
          }
        .label {
          color: var(--main-blue);
        }
      `];
  }

  @property()
  mode:MODE = "pair";

  render() {
      return html`
          <img-ui path="core/buttons/icon/circle-plus-blue.svg"></img-ui>
          <div class="label">${STR_PAIR}</div>
      `;
  }
}
