import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {arrayIndex} from "@utils/array";

@customElement('sidebar-widget-single-list')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              display: block;
          width: calc(100% - 4px);
          border-radius: 16px;
          border: solid 2px var(--Light_Blue_4);
          background-color: var(--white);
          }

          .list {
              display: flex;
              flex-direction: column;
          }

          ::slotted(*:not(:last-child)) {
              border-bottom: solid 1px var(--Light_Blue_4);
          }
        input {
            width: 460px; /*arbitrary amount to not go into rounded corners*/
          outline: none;
          border: none;
          font-size: 16px;
            text-align: center; 
        }

    `];
  }

  render() {
      return html`
          <div class="list">
              <slot></slot>
          </div>
      `
  }
}
