import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

@customElement('main-cards')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              display: flex;
              justify-content: center;
              overflow-x: hidden;
          }
          section {
              margin: 80px;
              display: flex;
              flex-wrap: wrap;
              justify-content: center;
              gap: 80px;
          }
    `];
  }

  render() {
      return html`
          <section>
              <slot></slot> 
          </section>
      `
  }
}
