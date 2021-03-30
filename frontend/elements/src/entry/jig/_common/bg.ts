import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";

@customElement("bg-jig")
export class _ extends LitElement {
  static get styles() {
    return [
        css`
            :host {
                display: block;
                background-color: var(--light-blue-3);
                width: 100vw;
                height: 100vh;
            }

            .right, .left {
                position: fixed;
                z-index: 0;
            }
            .right {
                top: -347px;
                right: -121px;

            }
            .left {
                top: 503px;
                left: 88px;
            }
            .content {
                position: fixed;
                z-index: 1;
                width: 100%;
                height: 100%;
            }
            `,
    ];
  }


  render() {
      return html`
          <img-ui class="right" path="entry/jig/bg/splash-right.png"></img-ui>
          <img-ui class="left" path="entry/jig/bg/splash-left.png"></img-ui>
          <div class="content">
              <slot></slot>
          </div>
      `;
  }
}
