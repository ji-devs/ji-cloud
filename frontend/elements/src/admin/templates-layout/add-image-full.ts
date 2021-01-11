import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('add-image-full')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .main-wrapper{
        padding:40px;
        position:relative;
    }
    .wrapper{
        display:flex;
       padding-top:40px;
     
    }


    ::slotted([slot=title]){
      
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