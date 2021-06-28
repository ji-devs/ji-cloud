import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

@customElement('flashcards-main')
export class _ extends LitElement {
  static get styles() {
      return [css`
      :host {
        position: absolute;
        top: 0;
        left: 0;
        display: flex;
        width: 100%;
        height: 100%;
        align-items: center;
        justify-content: center;
      }
        section {
          display: flex;
          gap: 56px;
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
