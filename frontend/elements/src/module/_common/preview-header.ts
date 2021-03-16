import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const STR_TITLE = "Preview Mode";

@customElement('module-preview-header')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              display: flex;
              width: 100%;
              height: 112px;
              box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
              background-color: var(--white);
              justify-content: space-between;
              align-items: center;
          }
          .btn, .nav {
              z-index: 1;
          }
          .title {
              position: absolute;
              height: 112px;
              line-height: 112px;
              left: 0;
              top:0;
              font-size: 28px;
              text-align: left;
              color: var(--dark-blue-4);
              width: 100%;
              text-align: center;
          }
    `];
  }

  render() {
      return html`
          <div class="title">${STR_TITLE}</div>
          <div class="nav">
              <slot name="nav"></slot>
          </div>
          <div class="btn"><slot name="btn"></slot></div>

      `
  }
}
