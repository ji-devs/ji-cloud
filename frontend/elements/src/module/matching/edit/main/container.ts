import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

@customElement('matching-main')
export class _ extends LitElement {
  static get styles() {
      return [css`
      :host {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        height: 100%;
      }

      .top {
        margin-top: 38rem;
        gap: 164rem;
      }
      .bottom {
        margin-bottom: 38rem;
        gap: 38rem;
      }
        section {
          display: flex;
        }
        .floating {
          position: absolute;
          top: 0;
          left: 0;
        }
    `];
  }

  render() {
      return html`
        <section class="top">
      	  <slot name="top"></slot>
        </section>
        <section class="bottom">
      	  <slot name="bottom"></slot>
        </section>
        <div class="floating">
      	  <slot name="floating"></slot>
        </div>
      `
  }
}
