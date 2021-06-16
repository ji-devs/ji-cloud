import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const STR_EMPTY = "No preview yet";

@customElement('main-empty')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              width: 100%;
              height: 100%;
              display: flex;
              justify-content: center;
              align-items: center;
          }
          section.empty {
              display: flex;
              flex-direction: column;
              align-items: center;
          }
    `];
  }

  render() {
      return html`
          <section class="empty">
              <img-ui path="module/_groups/cards/edit/main/empty.svg"></img-ui>
              <div class="label">${STR_EMPTY}</div>
          </section>
      `
  }
}
