import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";


@customElement('module-header')
export class _ extends LitElement {
  static get styles() {
      return [css`
          .topRight {
              position: absolute;
              top: 21px;
              right: 40px;
              display: flex;
              gap: 24px;
              align-items: center;
          }
          .title {
              margin-top: 90px;
              font-size: 32px;
              font-weight: 900;
              letter-spacing: -0.32px;
              text-align: left;
              color: var(--dark-blue-4);
          }
    `];
  }

  @property()
  title:string = "";

  render() {
      const {title} = this;

      return html`
          <section>
                  <div class="topRight">
                      <slot name="controller"></slot>
                      <img-ui path="module/_common/header/jiggling-gear.png"></img-ui>
                  </div>
                  <div class="title">${title}</div>
                  <slot></slot>
          </section>
      `
  }
}
