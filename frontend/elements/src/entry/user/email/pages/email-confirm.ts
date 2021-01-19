import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement("page-email-confirm")
export class _ extends LitElement {
  static get styles() {
    return [css`
   
    .content-wrapper{
        display:flex;
        justify-content:center;
        width:100%;
        flex-direction: column;
        align-items:center;
    }
    .side-image{
        width: 100vw;
        height: 685px;
        background-color: #def4ff;
     
    }
    h1{
        font-size: 32px;
        font-weight: 900;
        color:#5662a3
    }
    .title{
        margin-bottom:80px;
    }
    ::slotted([slot=button]){
        margin-top:60px;
    }
   
    `];
  }

  @property()
  title:string = ""; 

  @property()
  hidden:boolean = true; 

  render() {

    const {title, hidden} = this;

    return html`
 <div class="wrapper">
  
  <div class="content-wrapper">
    <div class="title">
        <h1>${title}</h1>
        <slot name="subtitle"></slot>
        <slot name="button"></slot>

    </div>
  </div>
  <div class="side-image">
  </div>
</div>
  `;
  }
}