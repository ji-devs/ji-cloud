import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

@customElement('card-quiz-settings')
export class _ extends LitElement {
  static get styles() {
      return [css`
    `];
  }

  render() {
      return html`
              <slot></slot>
      `
  }
}
