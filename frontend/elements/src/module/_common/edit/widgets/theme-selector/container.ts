import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const STR_HEADER = "Select theme";

@customElement('theme-selector')
export class _ extends LitElement {
  static get styles() {
      return [css`
          .options {
              margin-top: 93px;
              display: grid;
              grid-template-columns: repeat(2, 1fr);
              grid-template-rows: 259px; /*jig selected height is 212px, plus 47px gap*/
          }
          header {
            font-size: 18px;
            color: var(--dark-gray-6);
          }
    `];
  }

  render() {

      return html`
          <header>${STR_HEADER}</header>
          <div class="options">
              <slot></slot>
          </div>
      `
  }
}
