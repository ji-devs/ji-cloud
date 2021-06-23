import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

@customElement('flashcards-main')
export class _ extends LitElement {
  static get styles() {
      return [css`
        :host {
          display: flex;
          gap: 56px;
        }
    `];
  }

  render() {
      return html`
      	<slot></slot>
      `
  }
}
