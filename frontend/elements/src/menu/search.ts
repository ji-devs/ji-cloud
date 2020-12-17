import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('search-menu')
export class _ extends LitElement {
  static get styles() {
    return [css`
   main{
       
   }
    `];
  }

  @property()
  label: string = "";

  render() {

    const {label} = this;

    return html`
    <main class="">
   
    </main>
  `;
  }
}