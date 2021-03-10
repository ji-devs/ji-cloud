import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import "@elements/core/carousel/multi";
import "@elements/module/memory/edit/choose/card";


const STR_TITLE = "Create a Memory Game";
const STR_SUBTITLE = "What do you want to do?";

@customElement('choose-page')
export class _ extends LitElement {
  static get styles() {
      return [css`
          header {
              margin-left: 116px;
            }
            .title {
              font-family: Poppins;
              font-size: 40px;
              font-weight: 900;
              letter-spacing: -0.4px;
              text-align: left;
              color: var(--Orange);
            }
            .subtitle {
              font-family: Poppins;
              font-size: 24px;
              letter-spacing: normal;
              text-align: left;
              color: var(--Dark_Gray_6);
            }
            :host {
                
            }

            .carousel {
                margin-top: 77px;
                height: 387px;
                width: 100%;
            }
    `];
  }

  render() {
      return html`
          <header>
              <div class="title">${STR_TITLE}</div>
              <div class="subtitle">${STR_SUBTITLE}</div>
          </header>
          <carousel-multi class="carousel">
              <slot></slot>
          </carousel-multi>
      `
  }
}
