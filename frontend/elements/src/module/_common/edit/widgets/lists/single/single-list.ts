import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {arrayIndex} from "@utils/array";

const STR_ADD_WORDS = "Add Your Words";
const STR_INPUT_FOOTER = "2 to 14 words";

@customElement('sidebar-widget-single-list')
export class _ extends LitElement {
  static get styles() {
      return [css`

          header {
              display: flex;
              justify-content: space-between;
          }

          .input-buttons {
              margin-top: 34px;
              margin-bottom: 24px;
              display: flex;
              justify-content: space-between;
          }

          .input-footer {
              margin-top: 24px;
              font-size: 16px;
              text-align: center;
              color: var(--light-blue-5);
          }

          .list {
              width: calc(100% - 4px);
              border-radius: 16px;
              border: solid 2px var(--light-blue-4);
              background-color: var(--white);
              display: flex;
              flex-direction: column;
          }

          .list > ::slotted(*:not(:last-child)) {
              border-bottom: solid 1px var(--light-blue-4);
          }
          .done-btn {
              display: flex;
              justify-content: flex-end;
              margin-top: 16px;
              padding-bottom: 40px;
          }

    `];
  }

  render() {
      return html`
          <header>
              <div>${STR_ADD_WORDS}</div>
              <div><slot name="clear"></slot></div>
          </header>
          <div class="input-buttons">
              <slot name="input-buttons"></slot>
          </div>
          <div class="list">
              <slot></slot>
          </div>
          <div class="input-footer">${STR_INPUT_FOOTER}</div>
          <div class="done-btn">
              <slot name="done-btn"></slot>
          </div>
      `
  }
}
