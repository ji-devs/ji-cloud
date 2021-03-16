import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const STR_EMPTY = "No preview yet";

@customElement('step1-main-duplicate')
export class _ extends LitElement {
  static get styles() {
      return [css`
          :host {
              display: flex;
              justify-content: center;
          }
          section {
              margin: 80rem;
              display: grid;
              grid-template-columns: repeat(3, 350rem);
              gap: 80rem;
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
