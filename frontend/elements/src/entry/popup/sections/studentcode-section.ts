import { MEDIA_UI } from '@utils/path';
 import { LitElement, html, css, customElement, property } from 'lit-element';
 export type Image = "Illustration_JIG_Sad_1.png.png" | "Illustration_JIG_Sad_1.png.png"| "Illustration_JIG_Sad_1.png.png" ;

  @customElement('studentcode-section')
 export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;
       margin-left:170px;
       margin-top:130px;
    }
   main{
    width: 760px;
    height: 462px;
    position:relative;
  }
    
  
  ::slotted([slot=title]){
    position:relative;
    text-align: center;
    top:86px;


  }
  
  ::slotted([slot=img]){
    bottom:-5px;
    left:0px;
     position:absolute;
  }
 img-ui{
    bottom:-5px;
    left:0px;
     position:absolute;

  }
  ::slotted([slot=baloon]){
    width: 216.6px;
    height: 143.1px;
     bottom:40px;
     left:120px;
      position:absolute;
  }
  ::slotted([slot=square]){
     margin-left:32px;
     display:block;
  }

    `];
  }
  @property()
  image:string = ""; 


  render() {
    const {image} = this;
    return html`
    <main>
        <slot  name="title"></slot>

          <div class="inside-wrapper"> 
          <slot name="square"></slot>
          </div>
         <slot name="img"></slot>

       

    </main>
  `;
  }
 }