import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

@customElement('card-quiz-main')
export class _ extends LitElement {
  static get styles() {
      return [css`
      :host {
        display: flex;
        width: 100%;
        height: 100%;
        align-items: center;
        justify-content: center;
      }
      section {
        display: flex;
        flex-direction: column;
        align-items: center;
      }

      .target {
        margin-bottom: 105rem;
      }
        .options {
          display: flex;
          gap: 80rem;
        }
    `];
  }

  render() {
      return html`
        <section>
          <div class="target">
            <slot name="target"></slot>
          </div>
          <div class="options">
            <slot name="options"></slot>
          </div>
        </section>
      `;
  }
}
