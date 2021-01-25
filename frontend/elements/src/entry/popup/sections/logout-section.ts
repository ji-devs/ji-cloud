import { MEDIA_UI } from '@utils/path';
 import { LitElement, html, css, customElement, property } from 'lit-element';
  @customElement('logout-section')
 export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
       display:flex;
    }
   main{
    width: 576px;
    height: 352px;
    position:relative;

  }
    
  
  ::slotted([slot=title]){
    margin-left:40px;
    margin-top:150px;
    display:block;
  }
  
  ::slotted([slot=line]){
    margin-left:40px;
    margin-top:7px;
    display:block;
  }
  ::slotted([slot=button]){
    right:40px;
    bottom:40px;
    // display:block;
    position:absolute;

  }

  ::slotted([slot=textbutton]){
    left:40px;
    bottom:50px;
     position:absolute;

  }

   img-ui{
    width: 193px;
    height: 118px;
    position:absolute;
    right:104px;

   }

  .lines{
    margin-top:100px;

  }
 

    `];
  }
  render() {
    const {} = this;
    return html`
    <main>
    <img-ui path="yellow_Illustration.png"></img-ui>
        <slot  name="title"></slot>
        <div class="lines">
       <slot name="line"></slot>
       </div>
         <div class="inside-wrapper">
         <slot name="button"></slot>
         <slot name="textbutton"></slot>

         </div>

    </main>
  `;
  }
 }