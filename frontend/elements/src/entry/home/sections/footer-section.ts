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
   #titleJi{
   }
  
   #titles{
     display:flex;
   }
   ::slotted([slot=footer-column]){
    margin-left:90px;
  }
  ::slotted([slot=titleAbout]){
    margin-left:600px;
    place-items::right;

  }
  ::slotted([slot=titleJi]){
    margin-left:80px;
  }
  ::slotted([slot=button]){
    margin-top:80px;
  }

  ::slotted([slot=kidsafe]){
    margin-left:100px;
    margin-bottom:99px;
 
  }

  
 
  }
    `];
  }
  render() {
    const {} = this;
    return html`
    <main>
    <div id="titles">  
      <slot  name="titleJi"></slot>
    <slot  name="titleAbout"></slot>
    </div>
         <div class="inside-wrapper">
         <slot id="footer-column" name="footer-column"></slot>
         <slot name="button"></slot>
         </div>

          <slot name="kidsafe"></slot>
    </main>
  `;
  }
 }