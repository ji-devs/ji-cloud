import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/images/ui";

@customElement('header-blue')
export class _ extends LitElement {
  static get styles() {
    return [css`

   main{
    width:100%;
    height:512px;
    background-color: #6ca1fc;
    position:relative; 
    display:flex;
    justify-content:center;
    align-items:center;
    flex-direction:column;
   }
   .magnifyer{
       position:absolute;
       top:0;
       left:0;
       z-index:1;
   }
   .jiggling{
     position:absolute;
     bottom:0;
     right:30px;
   }
   ::slotted(*){
     z-index:2
   }
   .wrapper{
     display:flex;
   }
   ::slotted([slot="subtitle"]:last-of-type){
     margin-left:8px;
     margin-bottom:64px;
   }
   ::slotted([slot="undertext"]){
     z-index:1;
   }
   .undertext{
     display:flex;
   }
 
   
  
 
    `];
  }


  render() {

    const {} = this;

    return html`
    <main>
        <img-ui path="magnifyer.png" class="magnifyer"></img-ui>
        <img-ui path="group-13562.svg" class="jiggling"></img-ui>
        <img-ui path="logo-yellow.svg"></img-ui>
        <div class="wrapper">
        <slot name="subtitle"></slot>
        </div>
        <slot name="search"></slot>
        <div class="undertext">
          <slot name="undertext"></slot>
        </div>
    </main>
  `;
  }
}