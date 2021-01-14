import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";

export type Mode = "confirmation" | "sendagain";

@customElement('confirmation-button')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
        box-shadow: 2px 2px 3px 0 rgba(0, 0, 0, 0.16);
        border-radius: 14px;
        background-color: #72cb91;
        padding: 16px 0 16px 22px;
        align-items:center;
        width: 374px;
        justify-items:center;
    }
    span{
        display:flex;
        align-items:center;
        color: #ffffff;
    }
    img-ui{
        margin-right: 16px;

    }
    p{
      color: #5590fc;
      margin-top:0;
      margin-bottom:0;
      cursor:pointer;
    
  }
    
  
   
    `];
  }

  @property()
  label:string = ""; 

  @property()
  path:string = ""; 
  
  @property()
    mode: Mode = "sendagain";

  render() {

    const {label, path, mode} = this;

    return html`

        ${mode === "confirmation" ? confirmation() : sendagain()}
   
      

        
      

  `;
  }
}


function confirmation() {
    return html`  <div class="wrapper">
    <span class="flex items-center font-sans font-normal text-lg"><img-ui path="icon-sent.svg" alt=""></img-ui>We have just sent you another email</span>

  </div>`
  
}

function sendagain() {
  return html`  <p>I didn’t receive an email, Please send again</p>`

}


