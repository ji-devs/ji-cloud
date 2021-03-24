import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";

@customElement('card-text')
export class _ extends LitElement {
  static get styles() {
      return [css`

    `];
  }

  @property()
  value:string = "";

  render() {
      const {value} = this;

      return html`${value}`
  }
}
