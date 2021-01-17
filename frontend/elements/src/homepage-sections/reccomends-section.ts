import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('reccomends-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;

    }
   main{
    width: 1920px;
     height: 287px;
    background-color:#d8e7fa;
   }
   
   
 
    `];
  }



  render() {

    const {} = this;

    return html`
    <main>
     <div class="inside-wrapper">
        <slot name="icon-title-paragraph"></slot>
    </div>
    </main>
  `;
  }
}