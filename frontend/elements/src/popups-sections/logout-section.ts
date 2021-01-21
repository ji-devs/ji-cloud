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
    //  background-image: url("Strip_Background_Jigglings@2x.jpg");
    width: 576px;
    height: 352px;
   }
   
   ::slotted([slot=title]){
   margin-top:89px;
   margin-left:40px;
   
   }
   img-ui{
    width: 193px;
    height: 118px;

   }
   ::slotted([slot=button]){
     display:block;
     margin-left:410px;
  }
 
    `];
  }



  render() {

    const {} = this;

    return html`
    <main>
    <slot  name="title"></slot>
    <slot name="line"></slot>
    <slot name="cancel"></slot>
    <slot name="button"></slot>
<img-ui path="yellow_Illustration.png"><img-ui>
   
    </main>
  `;
  }
}