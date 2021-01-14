  import { MEDIA_UI } from '@utils/path';
 import { LitElement, html, css, customElement, property } from 'lit-element';
  @customElement('footer-section')
 export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;
      
    }
   main{
    background-color: #192150;
   }
   
   #title{

   }
    `];
  }



  render() {

    const {} = this;

    return html`
    <main>
   
         <div class="inside-wrapper">
         <slot name="footer-column"></slot>
         </div>
         <slot name="kidsafe"></slot>
    </main>
  `;
  }
 }
