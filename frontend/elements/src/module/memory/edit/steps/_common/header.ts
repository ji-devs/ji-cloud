import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const STR_LABEL = "Create a Memory Game";

@customElement('steps-header')
export class _ extends LitElement {
  static get styles() {
      return [css`
          .topRight {
              position: absolute;
              top: 0;
              right: 40px;
              display: flex;
              gap: 24px;
              align-items: center;
          }
          .label {
              margin-top: 90px;
          font-size: 32px;
          font-weight: 900;
          letter-spacing: -0.32px;
          text-align: left;
          color: var(--Dark_Blue_4);
        }
    `];
  }

  render() {
      return html`
          <section>
                  <div class="topRight">
                      <slot name="controller"></slot>
                      <img-ui path="module/_common/header/jiggling-gear.png"></img-ui>
                  </div>
              <div class="label">${STR_LABEL}</label>
          </section>
      `
  }
}
