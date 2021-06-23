import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const STR_HEADER = "Select how the player will view the cards";

@customElement('flashcards-settings')
export class _ extends LitElement {
  static get styles() {
      return [css`
          .options {
              margin-top: 93px;
              display: grid;
              grid-template-columns: repeat(2, 1fr);
          }
          header {
            font-size: 18px;
            color: #4a4a4a;
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
