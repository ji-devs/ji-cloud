import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('jiggling-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;
     
    }
   main{
     
   }
   
   #title{
    font-size: 64px;
    font-weight: 900;
    color:#5662a3;
    text-align: center;
   }
 
    `];
  }



  render() {

    const {} = this;

    return html`
    <main>
    <slot id="title" name="title"></slot>
    <div class="inside-wrapper">
        <slot name="icon-title-paragraph"></slot>
    </div>
    </main>
  `;
  }
}