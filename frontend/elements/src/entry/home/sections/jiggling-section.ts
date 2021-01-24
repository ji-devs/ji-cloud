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
     width:1920px;
     height:890px;
     background-image: url("Strip_Background_Jigglings@2x.jpg");
     background-repeat: no-repeat;
     background-attachment: fixed;
     background-position: center;

   }
   
   ::slotted([slot=title]){
    font-size: 64px;
    font-weight: 900;
    color:#5662a3;
    text-align: center;
    display:block;
   }
 
    `];
  }



  render() {

    const {} = this;

    return html`
    <main>
    <slot name="title"></slot>
    <div class="inside-wrapper">
        <slot name="icon-title-paragraph"></slot>
    </div>
    </main>
  `;
  }
}