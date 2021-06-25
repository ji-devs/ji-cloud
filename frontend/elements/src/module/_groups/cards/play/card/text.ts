import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import { styleMap } from 'lit-html/directives/style-map';

@customElement('card-text')
export class _ extends LitElement {
  static get styles() {
      return [css`
          span {
              font-family: var(--font-family, Poppins);
              font-size: var(--font-size, 40rem);
              color: var(--color, black);
              text-align: center;
          }
    `];
  }

  @property()
  value:string = "";

  render() {
    const { value} = this;

    return html`<span>${value}</span>`
  }
}
