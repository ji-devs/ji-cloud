import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('template-popups')
export class _ extends LitElement {
  static get styles() {
    return [css`


  .tamplate {
    background-color:#fff2cb; 
}

  .medium{
    width: 576px;
    height: 352px;
  }

  .peach{
    background-color:#fff2cb; 

  }
  img-ui{
      margin-top:24px;
      margin-left:520px;
      display:block;
  }
    `]
  }


  @property()
  color:string = ""; 
  @property()
  size:string = ""; 


  render() {

    const {color,size} = this;

    return html`
     <div class="template">
     <img-ui path="Icn_Delete_32.png"></img-ui>
      <slot name="content"></slot>
      </div>
        
  `;
  }
}

 