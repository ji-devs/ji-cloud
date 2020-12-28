import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('imagelabel-full')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .main-wrapper{
        padding:40px;
    }
    .wrapper{
        display:flex;
       padding-top:40px;
     
    }

    ::slotted([slot=left]){
      padding-right: 64px;
      border-right:solid 1px #e5e7ef;
      height: 744px;
      
    }
    ::slotted([slot=middle]){
        padding-left:40px;
        margin-right:24px;
    }
    ::slotted([slot=right]){
      width:100%;
  }
   
    `];
  }

  render() {

    const {} = this;

    return html`
    <div class="main-wrapper">
        <slot name="title"></slot>
        <div class="wrapper">
            <slot name="left"></slot>
            <slot name="middle"></slot>
            <slot name="right"></slot>
        </div>
    </div>  
  `;
  }
}